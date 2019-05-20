//! A shared database connection pool that allows sharing database connections across
//! rocket routes. The default PgConnection is not thread-safe.
//!
//! This file is adapted from the following:
//! https://github.com/ghotiphud/rust-web-starter/blob/master/api_server/src/pg_pool.rs

use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

/// The connection type being abstracted
type ConnectionType = PgConnection;

/// A pool of database connections
pub type DBPool = Pool<ConnectionManager<ConnectionType>>;

/// Create a new database pool
pub fn connect(database_url: &str) -> DBPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create database pool")
}

/// Connection request guard type for rocket: a wrapper around the connection pool
pub struct Connection(PooledConnection<ConnectionManager<ConnectionType>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    /// Attempts to retrieve a single connection from the managed database pool. If
    /// no pool is currently managed, fails with an `InternalServerError` status. If
    /// no connections are available, fails with a `ServiceUnavailable` status.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<DBPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &ConnectionType.
impl Deref for Connection {
    type Target = ConnectionType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
