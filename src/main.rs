#[macro_use]
extern crate diesel;
extern crate actix_cors;
extern crate actix_web;

pub mod custom_types;
pub mod helpers;
pub mod models;
pub mod schema;
pub mod services;
pub mod user_management;

use crate::user_management::get::{get_latest_activity, verify_auth_token};
use crate::user_management::post::{create_new_project, user_login};
use actix_cors::Cors;
use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};

// here is the server live check funciton
async fn live() -> impl Responder {
    HttpResponse::Accepted().body("The server is live.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(&std::env::var("CORS_ALLOWED").unwrap())
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ]);
        App::new()
            .wrap(cors)
            .route("/live", web::get().to(live))
            .route("/verify_user", web::get().to(verify_auth_token))
            .route(
                "/register",
                web::post().to(user_management::post::user_register),
            )
            .route("/login", web::post().to(user_login))
            .route("/get_activity", web::get().to(get_latest_activity))
            .route("/generate_project", web::post().to(create_new_project))
    })
    .bind(("localhost", 8080))
    .expect("The server failed to start.")
    .run()
    .await
}
