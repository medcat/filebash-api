use rocket::{routes, Route};
mod guard;
mod session;

pub use self::guard::SessionGuard;

pub fn routes() -> Vec<Route> {
    routes![session::create]
}
