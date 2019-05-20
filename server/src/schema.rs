table! {
    course_plans (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    term_courses (id) {
        id -> Int4,
        term_id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    terms (id) {
        id -> Int4,
        course_plan_id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamptz,
        google_id -> Nullable<Varchar>,
    }
}

joinable!(course_plans -> users (user_id));
joinable!(term_courses -> terms (term_id));
joinable!(terms -> course_plans (course_plan_id));

allow_tables_to_appear_in_same_query!(
    course_plans,
    term_courses,
    terms,
    users,
);
