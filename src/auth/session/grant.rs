use super::access_fail;
use crate::auth::SessionGuard;
use crate::store::models::{Session, SessionKind};
use crate::store::Store;
use rocket::http::Status;
use rocket::response::{status, Responder};
use rocket::State;

#[post("/session/grant")]
pub fn apply(store: State<Store>, session: SessionGuard) -> impl Responder {
    let user = session.user();
    match store.with_connection(|conn| Session::create(conn, SessionKind::Grant, user.id, true)) {
        Ok(Some(session)) => status::Custom(
            Status::Created,
            json!({ "success": true, "result": session }),
        ),
        Ok(None) => access_fail(),
        Err(_) => access_fail(),
    }
}
