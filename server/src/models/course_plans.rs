// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::{DateTime, Utc};

use schema::*;
use super::users::User;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct CoursePlan {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Associations)]
#[belongs_to(User)]
#[table_name="course_plans"]
struct NewCoursePlan {
    pub user_id: i32,
}

/// Retrieve the default course plan for a given user
pub fn get_default(conn: &PgConnection, user: &User) -> QueryResult<CoursePlan> {
    CoursePlan::belonging_to(user)
        .first::<CoursePlan>(conn)
}

/// Retrieve a course plan for a given user based on the course plan identifier
pub fn get(conn: &PgConnection, course_plan_id: i32, user: &User) -> QueryResult<CoursePlan> {
    use schema::course_plans::dsl::{course_plans, user_id, id};

    course_plans.filter(user_id.eq(user.id))
        .filter(id.eq(course_plan_id))
        .first::<CoursePlan>(conn)
}

/// Inserts a new course plan in the database and returns the complete record
pub fn create(conn: &PgConnection, user: &User) -> QueryResult<CoursePlan> {
    let new_course_plan = NewCoursePlan {
        user_id: user.id,
    };

    diesel::insert_into(course_plans::table)
        .values(&new_course_plan)
        .get_result(conn)
}
