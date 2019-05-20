// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

// Required for rocket
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;
extern crate jsonwebtoken as jwt;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate juniper;
extern crate juniper_rocket;

extern crate dotenv;
extern crate chrono;

mod schema;
mod models;
mod graphql;
mod api;
mod template;

use std::env;

use dotenv::dotenv;

fn main() {
    // Load the environment from the .env configuration
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Ensure that the secret key is set
    let secret_key = env::var("SECRET_KEY")
        .expect("SECRET_KEY must be set");

    api::run_server(&database_url, secret_key, &[
        //TODO: Get these URLs from a configuration instead of hardcoding them
        "http://localhost:1234",
        "http://local.uwcourseplan.com:1234",
        "http://local.uwcourseplan.com:8000",
        "http://uwcourseplan.com",
    ]);
}
