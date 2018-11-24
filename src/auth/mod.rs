use rocket::{Route, routes};
mod session;

pub fn routes() -> Vec<Route> {
    routes![session::create]
}
