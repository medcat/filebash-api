use crate::store::Store;
use rocket::request::Form;
use rocket::response::Responder;
use rocket::State;

#[derive(FromForm)]
pub(super) struct SessionCreateData {
    user: String,
    pass: String,
}

#[post("/session/new", data = "<data>")]
pub(super) fn create<'r>(
    data: Form<SessionCreateData>,
    store: State<'r, Store>,
) -> impl Responder<'r> {

}
