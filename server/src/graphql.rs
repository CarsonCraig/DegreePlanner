//! Adapter to the GraphQL query format.
//!
//! NOTE: Names of fields should be camelCase, not snake_case to match JavaScript conventions
#![allow(non_snake_case)]

use juniper::{self, FieldResult};
use chrono::{DateTime, Utc};
use diesel::result::Error as QueryError;
use diesel::Connection;

use api::db;
use models::{users, course_plans, terms, term_courses};
use template::CoursePlanTemplate;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation)
}

/// Allows GraphQL objects to access shared data
pub struct Context {
    /// The database connection
    pub conn: db::Connection,
    pub user: users::User,
}

// Implement the marker trait to make our context usable by juniper
impl juniper::Context for Context {}

/// The root query
pub struct Query;

graphql_object!(Query: Context |&self| {
    description: "The root query of the schema"

    // Accessing the currently logged in user
    field me(&executor) -> User as "The currently logged in user" {
        let ctx = executor.context();
        ctx.user.clone().into()
    }

    field coursePlan(&executor, default: bool) -> FieldResult<CoursePlan> as "Query for a specific course plan" {
        assert!(default, "Use coursePlan(default: true) to query course plans");
        let ctx = executor.context();
        let course_plan = course_plans::get_default(&ctx.conn, &ctx.user)?;
        Ok(CoursePlan {course_plan})
    }
});

pub struct CoursePlan {
    course_plan: course_plans::CoursePlan,
}

graphql_object!(CoursePlan: Context |&self| {
    description: "A course plan"

    // Accessing the course plan id
    field id() -> i32 as "A unique identifier for the course plan" {
        self.course_plan.id
    }

    field terms(&executor) -> FieldResult<Vec<Term>> as "List of all the terms in this course plan" {
        let ctx = executor.context();
        let terms = terms::all(&ctx.conn, &self.course_plan)?;
        let mut gql_terms = Vec::new();
        for t in terms {
            gql_terms.push(Term {term: t});
        }
        Ok(gql_terms)
    }
});

pub struct Term {
    term: terms::Term,
}

graphql_object!(Term: Context |&self| {
    description: "A term in a given course plan"

    field id() -> i32 as "A unique term identifier" {
        self.term.id
    }

    field name() -> &str as "Name of the term in no particular format" {
        &self.term.name
    }

    field courses(&executor) -> FieldResult<Vec<TermCourse>> as "List of courses associated with this term" {
        let ctx = executor.context();
        let term_courses = term_courses::all(&ctx.conn, &self.term)?;
        let mut gql_term_courses = Vec::new();
        for t in term_courses {
            gql_term_courses.push(t.into());
        }
        Ok(gql_term_courses)
    }
});

pub struct DeletedTerm {
    term: terms::Term,
    courses: Vec<term_courses::TermCourse>,
}

graphql_object!(DeletedTerm: Context |&self| {
    description: "The term that was removed from the course plan"

    field id() -> i32 as "A unique term identifier" {
        self.term.id
    }

    field name() -> &str as "Name of the term in no particular format" {
        &self.term.name
    }

    field courses(&executor) -> FieldResult<Vec<TermCourse>> as "List of courses associated with this term" {
        let mut term_courses = Vec::new();
        for course in &self.courses {
            term_courses.push(TermCourse {id: course.id, termId: course.term_id, name: course.name.clone()});
        }
        Ok(term_courses)
    }
});

#[derive(Debug, GraphQLObject)]
/// A course associated with a particular term
pub struct TermCourse {
    /// A unique identifier for a course associated with a term
    pub id: i32,
    /// The identifier of the term that the course is associated with
    pub termId: i32,
    /// Name of the course in no particular format
    pub name: String,
}

impl From<term_courses::TermCourse> for TermCourse {
    fn from(term_courses::TermCourse {id, term_id, name, ..}: term_courses::TermCourse) -> Self {
        TermCourse {
            id,
            termId: term_id,
            name,
        }
    }
}

#[derive(Debug, GraphQLObject)]
/// An application user
pub struct User {
    /// A unique identifier for the user
    pub id: i32,
    /// The full provided name of the user
    pub name: String,
    /// The email of the user
    pub email: String,
    /// The date that the user was created
    pub createdAt: DateTime<Utc>,
}

impl From<users::User> for User {
    fn from(users::User {id, name, email, created_at, ..}: users::User) -> Self {
        User {
            id,
            name,
            email,
            createdAt: created_at,
        }
    }
}

/// The input to the createCoursePlan mutation
#[derive(GraphQLInputObject)]
struct CreateCoursePlanInput {
    /// The program identifier (e.g. uw-software-engineering_2018-2019_stream-8)
    program: Option<String>,
    /// Data extracted from a user's transcript (JSON string)
    transcript: Option<String>,
}

/// All of the supported mutations
pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    description: "The available mutations of the schema"

    field createCoursePlan(&executor, params: CreateCoursePlanInput) -> FieldResult<CoursePlan> as "Create a new course plan for the currently logged in user" {
        // Enforce that only a single course plan per user is allowed right now
        let ctx = executor.context();
        let course_plan = course_plans::get_default(&ctx.conn, &ctx.user);
        match course_plan {
            Ok(_) => Err("Course plan already exists for user (limit of 1 per user for now)")?,
            Err(QueryError::NotFound) => {
                // No course plan found, good! That means we can create one.
            },
            Err(err) => Err(err)?,
        }

        // Create a new course plan
        let template = match (&params.program, &params.transcript) {
            //TODO: Match up the person's transcript with their program to get a full course plan
            (Some(program), Some(transcript)) if !program.is_empty() && !transcript.is_empty() => {
                Err("Program template and transcript matching is currently unsupported")?
            },
            (Some(program), None) if !program.is_empty() => {
                CoursePlanTemplate::from_template(program)?
            },
            (None, Some(transcript)) if !transcript.is_empty() => transcript.parse()?,
            // Return the blank course plan as is
            _ => {
                let course_plan = course_plans::create(&ctx.conn, &ctx.user)?;
                return Ok(CoursePlan {course_plan});
            },
        };

        // Any error in this transaction will cause all of the changes to be rolled back
        let course_plan = ctx.conn.transaction::<_, QueryError, _>(|| {
            let course_plan = course_plans::create(&ctx.conn, &ctx.user)?;
            for term in template.terms {
                let dbterm = terms::create(&ctx.conn, &course_plan, term.name)?;
                for course in term.courses {
                    term_courses::create(&ctx.conn, &dbterm, course.name)?;
                }
            }

            Ok(course_plan)
        })?;

        Ok(CoursePlan {course_plan})
    }

    field createTerm(&executor, coursePlanId: i32, name: String) -> FieldResult<Term> as "Create a new term for a specified course plan" {
        let ctx = executor.context();
        let course_plan = course_plans::get(&ctx.conn, coursePlanId, &ctx.user)?;
        let term = terms::create(&ctx.conn, &course_plan, name)?;
        Ok(Term {term})
    }

    field deleteTerm(&executor, termId: i32) -> FieldResult<DeletedTerm> as "Remove a specified term and all its associated courses from a course plan" {
        let ctx = executor.context();
        if terms::belongs_to_user(&ctx.conn, termId, &ctx.user)? {
            let term = terms::get(&ctx.conn, termId)?;
            let deleted_term = DeletedTerm {
                term: term.clone(),
                courses: term_courses::all(&ctx.conn, &term)?
            };
            term_courses::delete_all(&ctx.conn, termId)?;
            let count = terms::delete(&ctx.conn, termId)?;
            return Ok(deleted_term);
        }

        Err(format!("Could not find term with ID {} for the currently logged in user", termId))?
    }

    field createTermCourse(&executor, termId:i32, name: String) -> FieldResult<TermCourse> as "Create a new course for a specified term" {
        let ctx = executor.context();
        if terms::belongs_to_user(&ctx.conn, termId, &ctx.user)? {
            let term = terms::get(&ctx.conn, termId)?;
            let term_course = term_courses::create(&ctx.conn, &term, name)?;
            return Ok(term_course.into())
        }

        Err(format!("Could not find term with ID {} for the currently logged in user", termId))?
    }

    field deleteTermCourse(&executor, termCourseId: i32) -> FieldResult<TermCourse> as "Remove a specified course from a term in the logged in user's course plan" {
        let ctx = executor.context();
        if term_courses::belongs_to_user(&ctx.conn, termCourseId, &ctx.user)? {
            let deleted_course = term_courses::get(&ctx.conn, termCourseId)?;
            let count = term_courses::delete(&ctx.conn, termCourseId)?;
            return Ok(deleted_course.into());
        }

        Err(format!("Could not find course with ID {} for the currently logged in user", termCourseId))?
    }
});
