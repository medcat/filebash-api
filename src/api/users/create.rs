use crate::api::{database_fail, Code};
use crate::store::models::{Invite, User};
use crate::store::{Outcome, OutcomeExt, Secret, Store};
use postgres::transaction::Transaction;
use rocket::request::Form;
use rocket::response::status;
use rocket::response::Responder;
use rocket::State;
use rocket_contrib::uuid::Uuid;
use zxcvbn;

#[derive(Debug, Clone, FromForm)]
pub struct UserCreateData {
    pub email: String,
    pub password: String,
    #[form(field = "passwordConfirmation")]
    pub passsword_confirmation: String,
    pub invite: Uuid,
}

pub enum UpdateError {
    MismatchConfirmation,
    ShortPassword,
    InsecurePassword(Vec<&'static str>),
    InvalidInvite,
    InvalidEmail,
    EmailInUse,
    DatabaseError,
}

#[post("/users/new", data = "<data>")]
pub fn apply(data: Form<UserCreateData>, store: State<Store>) -> impl Responder<'static> {
    store
        .with_transaction(|trans| {
            let invite = otry!(find_invite(trans, data.invite));
            let user = otry!(create_user(trans, data.into_inner()));
            otry!(invite.update(trans, None, None, Some(Some(user.id)), Some(true)));
            Outcome::Commit(status::Accepted(Some(json!({
                "success": true,
                "result": None as Option<String>
            }))))
        })
        .map_rollback(update_error)
        .map_err(database_fail)
}

fn update_error(error: UpdateError) -> impl Responder<'static> + 'static {
    let errors = match error {
        UpdateError::MismatchConfirmation => json!([
            { "field": "password", "code": Code::FormFieldDoesNotMatch },
            { "field": "passwordConfirmation", "code": Code::FormFieldDoesNotMatch }
        ]),
        UpdateError::ShortPassword => json!([
            { "field": "password", "code": Code::FormFieldTooShort }
        ]),
        UpdateError::InvalidInvite => json!([
            { "field": "invite", "code": Code::FormFieldInvalid },
        ]),
        UpdateError::InvalidEmail | UpdateError::EmailInUse => json!([
            { "field": "email", "code": Code::FormFieldInvalid },
        ]),
        UpdateError::DatabaseError => json!([
            { "field": None as Option<String>, "code": Code::AccessDatabaseFail }
        ]),
        UpdateError::InsecurePassword(recommendations) => json!([
            { "field": "password", "code": Code::PasswordInsecure, "data": recommendations }
        ]),
    };

    status::BadRequest(Some(json!({
        "success": false,
        "errors": errors
    })))
}

fn find_invite(
    trans: &Transaction,
    invite: Uuid,
) -> Outcome<Invite, UpdateError, Box<dyn std::error::Error + Send + Sync>> {
    Outcome::from(Invite::find_update(trans, invite)).and_then(|opt| {
        opt.filter(Invite::usable)
            .unwrap_outcome_or(Outcome::Rollback(UpdateError::InvalidInvite))
    })
}

fn create_user(
    trans: &Transaction,
    mut data: UserCreateData,
) -> Outcome<User, UpdateError, Box<dyn std::error::Error + Send + Sync>> {
    if data.password != data.passsword_confirmation {
        Outcome::Rollback(UpdateError::MismatchConfirmation)
    } else if data.password.len() < 3 {
        Outcome::Rollback(UpdateError::ShortPassword)
    } else {
        otry!(validate_password(&data));
        let secret = otry!(Outcome::from(Secret::new(unsafe {
            data.password.as_bytes_mut()
        })));
        Outcome::from(User::create(trans, &data.email, secret))
            .and_then(|opt| opt.unwrap_outcome_or(Outcome::Rollback(UpdateError::DatabaseError)))
    }
}

fn validate_password(
    data: &UserCreateData,
) -> Outcome<(), UpdateError, Box<dyn std::error::Error + Send + Sync>> {
    // we unwrap here because the only error we can get from zxcvbn is a password that
    // is empty. we've already checked for that, though.
    let entropy = zxcvbn::zxcvbn(&data.password[..], &[&data.email[..]]).unwrap();
    if entropy.score < 3 {
        Outcome::Rollback(UpdateError::InsecurePassword({
            entropy
                .feedback
                .map(|feedback| {
                    let mut base = Vec::with_capacity(
                        feedback.suggestions.len() + feedback.warning.map(|_| 1).unwrap_or(0),
                    );

                    if let Some(warn) = feedback.warning {
                        base.push(warn);
                    }
                    base.extend(feedback.suggestions);

                    base
                })
                .unwrap_or_else(Vec::new)
        }))
    } else {
        Outcome::Commit(())
    }
}
