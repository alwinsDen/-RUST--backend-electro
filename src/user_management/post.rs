//post request for adding new user
extern crate actix_web;

use crate::helpers::module1::{argon2_hash_password, argon2_unhash, jwt_create, user_auth};
use crate::models::creation::{ActivityStruct, OrgProjectStruct, RegUsers};
use crate::models::read::GetUser;
use crate::schema::users::dsl::users;
use crate::services::module_1::db_connection;
use crate::user_management::structs::{
    LoginUserDeserialize, MessageResponse, MessageResponseJWT, ProjectJson, RegUserSerializer,
};
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Responder};
use diesel::RunQueryDsl;

//REGISTER USER API
pub async fn user_register(request: web::Json<RegUserSerializer>) -> impl Responder {
    let user_config = RegUsers {
        email: String::from(&request.email),
        password: argon2_hash_password(&request.password),
        id: uuid::Uuid::new_v4(),
        username: String::from(&request.username),
    };
    let diesel_run = diesel::insert_into(users)
        .values(&user_config)
        .execute(&db_connection());
    match diesel_run {
        Ok(_) => HttpResponse::Accepted().json(MessageResponse {
            message: "Account Created!",
        }),
        Err(err) => HttpResponse::NotAcceptable().json(MessageResponse {
            message: err.to_string().as_str(),
        }),
    }
}

// LOGIN USER API
pub async fn user_login(request: web::Json<LoginUserDeserialize>) -> Result<HttpResponse, Error> {
    let existing_users = &GetUser::verify_login(&request);
    let argon_closure = |_| {
        // NOTE: as_ref is used to convert &Result<t,e> to Result<&t,&e>
        return match argon2_unhash(
            request.password.to_string(),
            &existing_users.as_ref().unwrap()[0].password,
        ) {
            Ok(_) => Ok(*&existing_users.as_ref().unwrap()[0].id),
            Err(_) => Err(&"PASSWORD UN-HASHING FAILED."),
        };
    };
    match &existing_users.as_ref().and_then(argon_closure) {
        Ok(obj) => {
            ActivityStruct {
                user_id: *obj,
                related_info: "The user has been logged in.".to_string(),
            }
            .save();
            return Ok(HttpResponse::Accepted().json(MessageResponseJWT {
                message: "User Verified.",
                token: &*jwt_create(&request.email),
            }));
        }
        Err(_) => Err(error::ErrorForbidden("User has not been verified.")),
    }
}

// CREATE PROJECT
pub async fn create_new_project(
    headers: HttpRequest,
    json_request: web::Json<ProjectJson>,
) -> Result<HttpResponse, Error> {
    user_auth(headers).and_then(|user| {
        OrgProjectStruct {
            user_id: user.id,
            related_files: vec!["".to_string()],
            display_name: (&json_request.project_name).to_string(),
        }
        .create();
        return Ok(HttpResponse::Accepted().json(MessageResponse {
            message: "Project Successfully created.",
        }));
    })
}
