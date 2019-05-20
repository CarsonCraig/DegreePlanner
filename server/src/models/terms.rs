// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc};

use schema::*;
use super::course_plans::CoursePlan;
use super::users::User;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[belongs_to(CoursePlan)]
pub struct Term {
    pub id: i32,
    pub course_plan_id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Associations)]
#[belongs_to(CoursePlan)]
#[table_name="terms"]
struct NewTerm {
    pub course_plan_id: i32,
    pub name: String,
}

/// Retrieve the list of terms for a given course plan
pub fn all(conn: &PgConnection, course_plan: &CoursePlan) -> QueryResult<Vec<Term>> {
    use schema::terms::dsl::*;

    terms.filter(course_plan_id.eq(course_plan.id))
        .load::<Term>(conn)
}

/// Retrieve a term for a given user's course plan based on the term identifier
pub fn get(conn: &PgConnection, term_id: i32) -> QueryResult<Term> {
    use schema::terms::dsl::terms;

    terms.find(term_id)
        .first::<Term>(conn)
}

/// Checks if the specified term belongs to the given user
pub fn belongs_to_user(conn: &PgConnection, term_id: i32, user: &User) -> QueryResult<bool> {
    use schema::terms::dsl::{terms, id as term_id_column};
    use schema::course_plans::dsl::course_plans;
    use schema::users::dsl::{users, id as user_id};

    terms.inner_join(course_plans.inner_join(users))
        .filter(term_id_column.eq(term_id))
        .filter(user_id.eq(user.id))
        .select(term_id_column)
        .get_result(conn)
        .optional()
        .map(|res: Option<i32>| res.is_some())
}

/// Inserts a new term in the database and returns that record
pub fn create(conn: &PgConnection, course_plan: &CoursePlan, name: String) -> QueryResult<Term> {
    let new_term = NewTerm {
        course_plan_id: course_plan.id,
        name,
    };

    diesel::insert_into(terms::table)
        .values(&new_term)
        .get_result(conn)
}

/// Delete a term in a specified course plan
pub fn delete(conn: &PgConnection, term_id: i32) -> QueryResult<usize> {
    use schema::terms::dsl::terms;

    diesel::delete(terms.find(term_id))
        .execute(conn)
}
