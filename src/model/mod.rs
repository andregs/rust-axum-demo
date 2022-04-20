use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LoginOk {
    pub token: Token,
}

pub type Token = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct AuthOk {
    pub username: String,
}
