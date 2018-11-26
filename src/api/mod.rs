use rocket::http::Status;
use rocket::response::status;
use rocket::response::Responder;
use rocket::Route;
use std::error::Error;

mod code;
mod users;

pub use self::code::Code;

pub fn routes() -> Vec<Route> {
    routes![users::create::apply]
}

fn database_fail(error: Box<dyn Error + Send + Sync>) -> impl Responder<'static> + 'static {
    status::Custom(
        Status::InternalServerError,
        json!({
            "success": false,
            "errors": [
                { "field": None as Option<String>, "code": Code::AccessDatabaseFail, "description": error.to_string() }
            ]
        }),
    )
}
