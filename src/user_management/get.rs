extern crate actix_web;
extern crate chrono;
extern crate mime;

use crate::helpers::module1::user_auth;
use crate::models::read::GetActivityTracker;
use crate::schema::activity_tracker::dsl::activity_tracker;
use crate::schema::activity_tracker::user_id;
use crate::services::module_1::db_connection;
use crate::user_management::structs::{ActivityResponse, MessageResponse};
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use diesel::{EqAll, QueryDsl, RunQueryDsl};
use futures_util::future::err;

pub async fn get_latest_activity(
    request: HttpRequest,
    // json: web::Json<test>,
    // params: web::Query<test>,
) -> Result<HttpResponse, Error> {
    user_auth(request).and_then(|user| {
        let activity_data = activity_tracker
            .filter(user_id.eq_all(user.id))
            .load::<GetActivityTracker>(&db_connection())
            .unwrap();
        let incoming_activity = &activity_data[activity_data.len() - 1];
        return Ok(HttpResponse::Accepted().json(ActivityResponse {
            created_at: incoming_activity.created_at.to_string(),
            related_info: format!("{}", &incoming_activity.related_info),
        }));
    })
}

// authenticate user token
pub async fn verify_auth_token(request: HttpRequest) -> Result<HttpResponse, Error> {
    user_auth(request)
        .and_then(|user| {
            return Ok(HttpResponse::Accepted().json(user.serilize()));
        })
        .or_else(|_| return Err(error::ErrorForbidden("User verification failed.")))
}
