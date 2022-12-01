extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RegUserSerializer {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserDeserialize {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse<'a> {
    pub message: &'a str,
}

#[derive(Debug, Serialize)]
pub struct MessageResponseJWT<'a> {
    pub message: &'a str,
    pub token: &'a str,
}

#[derive(Serialize)]
pub struct ActivityResponse {
    pub created_at: String,
    pub related_info: String,
}

#[derive(Deserialize)]
pub struct ProjectJson {
    pub project_name: String,
}
