use super::{access_fail, authentication_fail};
use crate::store::models::User;
use crate::store::Store;
use rocket::request::Form;
use rocket::response::Responder;
use rocket::State;

#[derive(FromForm)]
pub struct SessionCreateData {
    user: String,
    pass: String,
}

#[post("/session/new", data = "<data>")]
pub fn apply<'r>(data: Form<SessionCreateData>, store: State<'r, Store>) -> impl Responder<'r> {
    let data = data.into_inner();

    let user = store
        .with_connection(|conn| User::find_email(conn, &data.user))
        .map(|u| u.filter(|user| user.pass().verify(data.pass.into_bytes()).unwrap_or(false)));

    match user {
        Ok(Some(_user)) => unimplemented!(),
        Ok(None) => authentication_fail(),
        Err(_) => access_fail(),
    }
}
