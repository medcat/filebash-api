use crate::store::models::{Session, User};
use crate::store::Store;
use regex::Regex;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::State;
use rocket::request::{self, FromRequest, Request};
use uuid::Uuid;

pub struct SessionGuard(Session, User);

#[derive(Debug)]
pub enum SessionTokenError {
    Missing,
    Invalid,
    BadCount,
    BadConnect,
}

lazy_static! {
    static ref AUTHORIZATION_REGEX: Regex =
        Regex::new(r"^Bearer ([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})$")
            .unwrap();
}

impl<'a, 'r> FromRequest<'a, 'r> for SessionGuard {
    type Error = SessionTokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<SessionGuard, Self::Error> {
        let authorizations = request.headers().get("authorization").collect::<Vec<_>>();
        match authorizations.len() {
            0 => return Outcome::Failure((Status::Unauthorized, SessionTokenError::Missing)),
            1 => {}
            _ => return Outcome::Failure((Status::Unauthorized, SessionTokenError::BadCount)),
        }

        let token = match extract_token(authorizations[0]) {
            None => return Outcome::Failure((Status::Unauthorized, SessionTokenError::Missing)),
            Some(tok) => tok,
        };

        match request.guard::<State<Store>>() {
            Outcome::Failure(_) => {
                Outcome::Failure((Status::InternalServerError, SessionTokenError::BadConnect))
            }
            Outcome::Forward(_) => unreachable!(),
            Outcome::Success(state) => match Session::find_with_user(token, &*state) {
                Ok(Some((session, user))) => Outcome::Success(SessionGuard(session, user)),
                Ok(None) => Outcome::Failure((Status::Unauthorized, SessionTokenError::Missing)),
                Err(_) => Outcome::Failure((Status::Unauthorized, SessionTokenError::BadConnect)),
            },
        }
    }
}

fn extract_token(content: &str) -> Option<Uuid> {
    AUTHORIZATION_REGEX
        .captures(content)
        .and_then(|cap| cap.get(0))
        .and_then(|m| Uuid::parse_str(m.as_str()).ok())
}
