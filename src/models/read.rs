extern crate diesel;

use crate::schema::users::dsl::users;
use crate::schema::users::email;
use crate::services::module_1::db_connection;
use crate::user_management::structs::LoginUserDeserialize;
use diesel::{EqAll, QueryDsl, Queryable, RunQueryDsl};
use serde::Serialize;

// getUser related Interface
#[derive(Queryable, Debug, Clone)]
pub struct GetUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct GetUserFormatted {
    pub username: String,
    pub email: String,
}

impl Iterator for GetUser {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.to_owned())
    }
}
impl GetUser {
    pub fn serilize(&self) -> GetUserFormatted {
        return GetUserFormatted {
            username: (&self.username).to_string(),
            email: (&self.email).to_string(),
        };
    }
    pub fn verify_login<'a>(
        request: &actix_web::web::Json<LoginUserDeserialize>,
    ) -> Result<Vec<GetUser>, &'a str> {
        match users
            .filter(email.eq_all(&request.email))
            .load::<GetUser>(&db_connection())
        {
            Ok(t) => {
                if t.len() > 0 {
                    return Ok(t);
                }
                return Err("NO SUCH USER EXISTS");
            }
            Err(_) => Err("GETTING USER VALUES FAILED."),
        }
    }
}

// getActivityTracker Interface
#[derive(Queryable, Debug)]
pub struct GetActivityTracker {
    pub id: i64,
    pub user_id: uuid::Uuid,
    pub related_info: String,
    pub created_at: chrono::NaiveDateTime,
}
