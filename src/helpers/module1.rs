// make sure a file doesn't cross 2K lines.
extern crate chrono;

use crate::custom_types::ElectroCaed;
use crate::helpers::structs::TokenClaims;
use crate::models::read::GetUser;
use crate::schema::users::dsl::users;
use crate::schema::users::email;
use crate::services::module_1::db_connection;
use actix_web::{error, Error, HttpRequest};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::prelude::*;
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

// function to hash the incoming password
pub fn argon2_hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("HASHING INCOMING PASSWORD FAILED.")
        .to_string();
    return hashed_password;
}

// function to un-hash the password
pub fn argon2_unhash<'a>(
    request_password: String,
    db_password: &String,
) -> argon2::password_hash::Result<()> {
    let parsed_password = PasswordHash::new(&db_password).expect("PASSWORD DECRYPTION FAILED");
    return Argon2::default().verify_password(request_password.as_bytes(), &parsed_password);
}

// function to create JWT TOKEN
pub fn jwt_create(user_email: &String) -> String {
    //The token will expire in 86400 seconds = 1 day
    let token_valid_seconds = 86_400i64;
    let current_date: DateTime<Utc> = Utc::now();
    let token_claims = TokenClaims {
        exp: current_date.timestamp() + token_valid_seconds,
        sub: user_email.to_string(),
    };
    let jwt_secret_key = std::env::var("JWT_SECRET").expect("ENV EXTRACTION FAILED.");
    let header = Header {
        kid: Some(format!("{}", uuid::Uuid::new_v4())),
        alg: Algorithm::HS512,
        ..Default::default()
    };
    return match encode(
        &header,
        &token_claims,
        &EncodingKey::from_secret(jwt_secret_key.as_ref()),
    ) {
        Ok(t) => t,
        Err(_) => panic!("JWT TOKEN CREATION FAILED."),
    };
}

// function to decode JWT token and return user data
pub fn user_auth(user_request: HttpRequest) -> Result<GetUser, Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT SECRET REALIZATION FAILED.");
    match decode::<TokenClaims>(
        user_request
            .headers()
            .get(ElectroCaed::Authorizations.get())
            .unwrap()
            .to_str()
            .unwrap(),
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(claims) => {
            let existing_users = users
                .filter(email.eq_all(claims.claims.sub))
                .load::<GetUser>(&db_connection())
                .unwrap();
            match existing_users.into_iter().next() {
                Some(t) => Ok(t),
                None => Err(error::ErrorForbidden("User doesn't exist.")),
            }
        }
        Err(_) => Err(error::ErrorNotAcceptable("Failed user validation.")),
    }
}
