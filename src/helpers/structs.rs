extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct  TokenClaims {
    pub sub: String,
    pub exp: i64
}