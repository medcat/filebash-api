#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Code {
    AccessDatabaseFail = 1,

    FormFieldDoesNotMatch = 2,
    FormFieldTooShort = 3,
    FormFieldInvalid = 4,
    PasswordInsecure = 5,
}
