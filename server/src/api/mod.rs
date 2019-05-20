pub mod db;

mod auth;
mod graphql;

use rocket::{
    self,
    http::Method,
};
use rocket_cors::{Cors, AllowedOrigins, AllowedHeaders};

use self::auth::SecretKey;

#[get("/")]
fn index() -> &'static str {
    // Return nothing
    ""
}

pub fn run_server(database_url: &str, secret_key: String, allowed_origins: &[&str]) {
    let conn = db::connect(database_url);

    let (allowed_origins, failed_origins) = AllowedOrigins::some(allowed_origins);
    assert!(failed_origins.is_empty());

    let options = Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .manage(conn)
        .manage(::graphql::schema())
        .manage(SecretKey(secret_key))
        .mount("/", routes![
            index,
            auth::google_auth,
            auth::logout,
            graphql::graphiql,
            graphql::get_graphql,
            graphql::graphql,
        ])
        .attach(options)
        .launch();
}
