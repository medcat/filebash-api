use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::JsonValue;

pub mod create;
pub mod grant;

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
