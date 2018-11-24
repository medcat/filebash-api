use crate::store::models::User;
use crate::store::Store;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{status, Responder};
use rocket::State;
use rocket_contrib::json::JsonValue;

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
    let data = data.into_inner();

    let user = User::find_email(&data.user, &store)
        .map(|u| u.filter(|user| user.pass().verify(data.pass.into_bytes()).unwrap_or(false)));

    match user {
        Ok(Some(_user)) => unimplemented!(),
        Ok(None) => authentication_fail(),
        Err(_) => access_fail(),
    }
}

fn authentication_fail() -> status::Custom<JsonValue> {
    status::Custom(
        Status::UnprocessableEntity,
        json!({
            "success": false,
            "errors": [
                { "field": "user", "error": "unknown user" },
                { "field": "pass", "error": "invalid password" }
            ]
        }),
    )
}

fn access_fail() -> status::Custom<JsonValue> {
    status::Custom(
        Status::InternalServerError,
        json!({
            "success": false,
            "errors": [
                { "field": None as Option<String>, "error": "internal server error" }
            ]
        }),
    )
}
