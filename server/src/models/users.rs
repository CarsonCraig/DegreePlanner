// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use diesel::result::Error as QueryError;
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc};

use schema::*;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub google_id: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
struct NewUser {
    pub name: String,
    pub email: String,
    pub google_id: String,
}

/// Retrieve the user with the given ID
pub fn get(conn: &PgConnection, id: i32) -> QueryResult<User> {
    use schema::users::dsl::users;

    users.find(id)
        .first::<User>(conn)
}

/// Attempts to find a user record based on their google_id or based on their email.
///
/// Valid scenarios for this method (in order of precedence):
///   1. A user associated with the given google_id is found
///   2. A user associated with the given email with a NULL google_id is found
///   3. No user is found, so one is created
/// All other scenarios are invalid and will result in an error.
///
/// Consequences of this:
///   * If there is one account associated with the google_id and another different account
///     associated with the email, the account associated with the google_id is returned
///     Note: See Edge Case 1. Intuition: logging in with Google, should return the account
///       associated with that Google account.
///   * Given a google_id, the email does not matter at all. It will not be updated at any point.
///   * The google_id for an account *cannot* be updated once the account is created (this may
///     be too strict and might have to be changed later)
///   * If the user with the given email has already logged in via a different service, their
///     account will be preserved and linked with google. (i.e. this formulation allows linking
///     additional accounts with an existing account automatically)
///     Note: From a security point of view, this requires that we trust our login services to
///       verify that the person owns that email. Without this protection, it is easy to
///       impersonate accounts by setting your email on the login service.
///
/// Example: Given the parameters (name: "S", email: "a@b.c", google_id: "123"), we are able to use
/// the email to find the record: {name: "S", email: "a@b.c", google_id: NULL}. Since google_id is
/// NULL, we'll set it to "123". If the stored google_id does not match the provided google_id,
/// this is an error.
///
/// ## Edge Case 1
///
/// 1. User registers with FB, token: 123, email: a@b
/// 2. Same user registers with Google, id: abc, email: b@c (alternate email)
/// 3. User changes their FB email to b@c
/// 4. Now two users (created in 1 & 2) are returned when you query for token: 123, email: b@c
pub fn get_or_create(conn: &PgConnection, name: &str, email: &str, google_id: &str) -> QueryResult<User> {
    assert!(!name.is_empty() && !email.is_empty() && !google_id.is_empty());

    //FIXME: Validate google_id: https://developers.google.com/identity/sign-in/web/backend-auth

    // Both email and google_id columns have unique constraints which means that only up to one of
    // the following queries will return a value. If both return a value, those values will be
    // the same user.

    match get_by_google_id(conn, google_id)? {
        Some(user) => Ok(user),
        None => {
            match get_by_email(conn, email)? {
                //TODO: Is there a better way to handle this error?
                Some(ref user) if user.google_id != Some(google_id.to_string()) => {
                    //FIXME: Add proper logging
                    eprintln!("User id {} changed their google ID", user.id);
                    Err(QueryError::NotFound)
                },
                Some(user) => Ok(user),
                None => {
                    let new_user = NewUser {
                        name: name.to_string(),
                        email: email.to_string(),
                        google_id: google_id.to_string(),
                    };

                    use schema::users;
                    diesel::insert_into(users::table)
                        .values(&new_user)
                        .get_result(conn)
                }
            }
        }
    }
}

fn get_by_email(conn: &PgConnection, email: &str) -> QueryResult<Option<User>> {
    use schema::users;

    // This can only return up to one user because of the unique constraint on the email column
    users::table.filter(users::email.eq(email))
        .first::<User>(conn)
        .optional()
}

fn get_by_google_id(conn: &PgConnection, google_id: &str) -> QueryResult<Option<User>> {
    use schema::users;

    // This can only return up to one user because of the unique constraint on the google_id column
    users::table.filter(users::google_id.eq(google_id))
        .first::<User>(conn)
        .optional()
}
