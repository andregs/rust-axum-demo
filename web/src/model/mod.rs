use lazy_regex::{lazy_regex, Lazy, Regex};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use validator::Validate;

// GDPR tip: do not derive Display or Debug on structs containing Personal Identifiable Info,
// to make it harder leaking them on logs.
// there's https://crates.io/crates/secrecy but I couldn't make it work with validator

#[derive(Serialize, Deserialize, Validate, PartialEq, Eq, Debug, Clone)]
pub struct Credentials {
    #[validate(
        length(min = 1, max = 42, message = "Username must be between 1 and 42 characters."),
        regex(
            path = "USERNAME_RE",
            message = "Username must be alphanumeric and it must start with a letter."
        )
    )]
    pub username: String,

    #[validate(length(min = 8, message = "Password must contain at least 8 characters."))]
    pub password: String,
}

static USERNAME_RE: Lazy<Regex> = lazy_regex!("^([A-Za-z]+)([0-9A-Za-z]*)$");

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LoginOk {
    pub token: Token,
}

pub type Token = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct AuthOk {
    pub username: String,
}
