use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};

pub enum Outcome<C, R, E> {
    Commit(C),
    Rollback(R),
    Err(E),
}

#[macro_export]
macro_rules! otry {
    ($value:expr) => {
        match $crate::store::outcome::Outcome::from($value) {
            $crate::store::outcome::Outcome::Commit(value) => value,
            $crate::store::outcome::Outcome::Rollback(r) => return $crate::store::outcome::Outcome::Rollback(r),
            $crate::store::outcome::Outcome::Err(e) => return $crate::store::outcome::Outcome::Err(e)
        }
    };
}

impl<C, R, E> Outcome<C, R, E> {
    pub fn and<C2>(self, other: Outcome<C2, R, E>) -> Outcome<C2, R, E> {
        match self {
            Outcome::Commit(_) => other,
            Outcome::Rollback(v) => Outcome::Rollback(v),
            Outcome::Err(v) => Outcome::Err(v),
        }
    }

    pub fn and_then<C2, F: FnOnce(C) -> Outcome<C2, R, E>>(self, then: F) -> Outcome<C2, R, E> {
        match self {
            Outcome::Commit(c) => then(c),
            Outcome::Rollback(v) => Outcome::Rollback(v),
            Outcome::Err(v) => Outcome::Err(v),
        }
    }

    pub fn map_err<E2, F: FnOnce(E) -> E2>(self, map: F) -> Outcome<C, R, E2> {
        match self {
            Outcome::Commit(v) => Outcome::Commit(v),
            Outcome::Rollback(v) => Outcome::Rollback(v),
            Outcome::Err(e) => Outcome::Err(map(e)),
        }
    }

    pub fn map_rollback<R2, F: FnOnce(R) -> R2>(self, map: F) -> Outcome<C, R2, E> {
        match self {
            Outcome::Commit(v) => Outcome::Commit(v),
            Outcome::Rollback(v) => Outcome::Rollback(map(v)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }

    pub fn map<C2, F: FnOnce(C) -> C2>(self, map: F) -> Outcome<C2, R, E> {
        match self {
            Outcome::Commit(v) => Outcome::Commit(map(v)),
            Outcome::Rollback(v) => Outcome::Rollback(v),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }
}

pub trait OutcomeExt<C, R, E> {
    fn unwrap_outcome_or(self, default: Outcome<C, R, E>) -> Outcome<C, R, E>;
    fn unwrap_outcome_or_else<F: FnOnce() -> Outcome<C, R, E>>(
        self,
        default: F,
    ) -> Outcome<C, R, E>;
}

impl<C, R, E> OutcomeExt<C, R, E> for Option<C> {
    fn unwrap_outcome_or(self, default: Outcome<C, R, E>) -> Outcome<C, R, E> {
        self.map(Outcome::Commit).unwrap_or(default)
    }

    fn unwrap_outcome_or_else<F: FnOnce() -> Outcome<C, R, E>>(
        self,
        default: F,
    ) -> Outcome<C, R, E> {
        self.map(Outcome::Commit).unwrap_or_else(default)
    }
}

impl<C, R, E> Into<Result<Option<C>, E>> for Outcome<C, R, E> {
    fn into(self) -> Result<Option<C>, E> {
        match self {
            Outcome::Commit(result) => Ok(Some(result)),
            Outcome::Rollback(_) => Ok(None),
            Outcome::Err(error) => Err(error),
        }
    }
}

impl<C, R, E> From<Result<C, E>> for Outcome<C, R, E> {
    fn from(result: Result<C, E>) -> Outcome<C, R, E> {
        match result {
            Ok(result) => Outcome::Commit(result),
            Err(error) => Outcome::Err(error),
        }
    }
}

impl<'r, C, R, E> Responder<'r> for Outcome<C, R, E>
where
    C: Responder<'r>,
    R: Responder<'r>,
    E: Responder<'r>,
{
    fn respond_to(self, request: &Request<'_>) -> Result<Response<'r>, Status> {
        match self {
            Outcome::Commit(result) => result.respond_to(request),
            Outcome::Rollback(result) => result.respond_to(request),
            Outcome::Err(result) => result.respond_to(request),
        }
    }
}
