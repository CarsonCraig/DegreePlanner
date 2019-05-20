use rocket::{
    State,
    Outcome,
    http::Status,
    request::{self, Request, FromRequest},
    response::Failure,
};
use rocket_contrib::Json;
use chrono::{DateTime, Utc, Duration};
use jwt::{self, Header, Validation};

use api::db;
use models::users;

#[derive(Debug, Deserialize)]
pub struct Auth {
    name: String,
    email: String,
    #[serde(rename = "googleId")]
    google_id: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Debug)]
pub struct SecretKey(pub String);

/// Represents the current session of a logged in user
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// The user ID of the currently logged in user
    pub user_id: i32,
    /// The expiry date of this session
    pub expiry: DateTime<Utc>,
}

// Defines that Session can be used as a request guard
impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = &'static str;

    // This function handles a number of different cases including:
    // 1. The authentication token not being provided via the Authorization header
    // 2. The Authorization header being in the wrong format
    // 3. The JWT token being invalid
    //
    // Each of these cases (and more) are handled by returning the right HTTP error code depending
    // on what went wrong. Some common error codes:
    //
    //     401 Unauthorized - no authorization token provided
    //     403 Forbidden - token provided, but invalid/expired
    //     500 Internal Server Error - something in our application was misconfigured
    //
    // If none of these error cases is encountered, the session is successfully decoded and
    // returned. Doing this as a request guard encapsulates all this logic in one place and makes
    // creating methods that require authentication as easy as using Session as a parameter.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let SecretKey(ref secret_key) = *request.guard::<State<SecretKey>>()
            .map_failure(|_| (Status::InternalServerError, "Application not configured correctly"))?;

        let keys: Vec<_> = request.headers().get("Authorization").collect();
        let token = match &keys[..] {
            // No access token provided
            [] => Outcome::Failure((Status::Unauthorized, "No authorization token provided"))?,
            [token] => match token.trim().split(' ').nth(1) {
                Some(token) => token,
                None => Outcome::Failure((
                    Status::BadRequest,
                    "Authorization header must be in the format 'Bearer <token>'",
                ))?,
            },
            _ => Outcome::Failure((Status::BadRequest, "Expected only a single token"))?,
        };

        let session: Session = match jwt::decode(token, secret_key.as_ref(), &Validation::default()) {
            Ok(jwt::TokenData {claims, ..}) => claims,
            // An authentication token was provided but it was invalid
            // This is the safest thing to do as a response here because if the session struct
            // changes, the user will simply be logged out instead of dealing with an error.
            Err(_) => Outcome::Failure((Status::Forbidden, "Invalid/expired token"))?,
        };

        // Check if the session is expired
        if Utc::now() > session.expiry {
            return Outcome::Failure((Status::Forbidden, "Invalid/expired token"));
        }

        Outcome::Success(session)
    }
}

#[post("/google_auth", data = "<auth>")]
pub fn google_auth(
    conn: db::Connection,
    secret_key: State<SecretKey>,
    auth: Json<Auth>,
) -> Result<Json<AuthResponse>, Failure> {
    let user = users::get_or_create(&conn, &auth.name, &auth.email, &auth.google_id)
        .map_err(|_| Failure(Status::InternalServerError))?;

    let SecretKey(ref secret_key) = *secret_key;
    let session = Session {
        user_id: user.id,
        expiry: Utc::now() + Duration::days(30),
    };
    let token = jwt::encode(&Header::default(), &session, secret_key.as_ref())
        .map_err(|_| Failure(Status::InternalServerError))?;

    Ok(Json(AuthResponse {
        token,
    }))
}

#[post("/logout")]
pub fn logout(_session: Session) {
    // Does nothing for now except getting the current session to make sure this route is only
    // called if the user is logged in. This route could one day be used to invalidate sessions.
    // TODO: Currently, since sessions are not invalided, tokens are actually valid until they
    // expire and logout does not do anything if the token is still stored.
}
