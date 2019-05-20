use rocket::{
    State,
    http::Status,
    response::{content, Failure},
};
use juniper_rocket::{GraphQLRequest, GraphQLResponse};

use api::db;
use api::auth::Session;

use models::users;

/// Opens the graphiql interface, not available when application compiled with --release
///TODO: Guard to only admin users and then make available even when compiled with --release
#[cfg(debug_assertions)]
#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
    // GraphiQL source code was literally being returned as an HTML string. See this file:
    // https://github.com/graphql-rust/juniper/blob/50a9fa31b673a8de9c12c457787d4251ef0f4af6/juniper/src/http/graphiql.rs
    // In order to add auth support, we took that HTML, customized it as needed, and made our own
    // version of it. This has the downside of not keeping up with the version of GraphiQL released
    // with Juniper. You should go update the versions of the cdn fetched JS as needed.
    content::Html(include_str!("graphiql.html")
        .replace("{graphql_url}", "/graphql")
        .replace("{google_client_id}", "988574817320-upn9d65cbmqvnol3h7fgro1cd7lo4l9h.apps.googleusercontent.com")
    )
}
#[cfg(not(debug_assertions))]
#[get("/graphiql")]
fn graphiql() -> &'static str {
    ""
}

#[get("/graphql")]
fn get_graphql() -> Failure {
    Failure(Status::MethodNotAllowed)
}

/// In GraphQL, everything is sent to a single endpoint via POST requests
#[post("/graphql", format = "application/json", data = "<request>")]
fn graphql(
    conn: db::Connection,
    schema: State<::graphql::Schema>,
    session: Session,
    request: GraphQLRequest,
) -> Result<GraphQLResponse, Failure> {
    // Fetch the currently logged in user. This should not fail unless something went horribly
    // wrong. That being said, it is better to return 403 Forbidden and force a logout than return
    // a 500 Internal Server Error
    let user = users::get(&conn, session.user_id)
        .map_err(|_| Failure(Status::Forbidden))?;
    Ok(request.execute(&schema, &::graphql::Context {
        conn,
        user,
    }))
}
