// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc};

use schema::*;
use super::terms::Term;
use super::users::User;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[belongs_to(Term)]
pub struct TermCourse {
    pub id: i32,
    pub term_id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Associations)]
#[belongs_to(Term)]
#[table_name="term_courses"]
struct NewTermCourse {
    pub term_id: i32,
    pub name: String,
}

/// Retrieve the list of courses for a given term of a specific course plan
pub fn all(conn: &PgConnection, term: &Term) -> QueryResult<Vec<TermCourse>> {
    use schema::term_courses::dsl::*;

    term_courses.filter(term_id.eq(term.id))
        .load::<TermCourse>(conn)
}

/// Retrieve a term course based on the term course identifier
pub fn get(conn: &PgConnection, term_course_id: i32) -> QueryResult<TermCourse> {
    use schema::term_courses::dsl::term_courses;

    term_courses.find(term_course_id)
        .first::<TermCourse>(conn)
}

/// Checks if a course belongs to the specified user
pub fn belongs_to_user(conn: &PgConnection, term_course_id: i32, user: &User) -> QueryResult<bool> {
    use schema::term_courses::dsl::{term_courses, id as term_course_id_column};
    use schema::terms::dsl::terms;
    use schema::course_plans::dsl::{course_plans, user_id};

    term_courses.inner_join(terms.inner_join(course_plans))
        .filter(term_course_id_column.eq(term_course_id))
        .filter(user_id.eq(user.id))
        .select(term_course_id_column)
        .get_result(conn)
        .optional()
        .map(|res: Option<i32>| res.is_some())
}

/// Inserts a term course in the database and returns that record
pub fn create(conn: &PgConnection, term: &Term, name: String) -> QueryResult<TermCourse> {
    let new_term_course = NewTermCourse {
        term_id: term.id,
        name: name,
    };

    diesel::insert_into(term_courses::table)
        .values(&new_term_course)
        .get_result(conn)
}

/// Delete a course from a specified term
pub fn delete(conn: &PgConnection, term_course_id: i32) -> QueryResult<usize> {
    use schema::term_courses::dsl::term_courses;

    diesel::delete(term_courses.find(term_course_id))
        .execute(conn)
}

/// Delete all the courses associated with a specified term
pub fn delete_all(conn: &PgConnection, term_id: i32) -> QueryResult<usize> {
    use schema::term_courses::dsl::{term_courses, term_id as term_id_column};

    diesel::delete(term_courses.filter(term_id_column.eq(term_id)))
        .execute(conn)
}
