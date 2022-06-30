use actix_identity::Identity;
use actix_web::{HttpResponse, Result, web};
use web::Json;

use crate::{
    configurations::db::Pool,
    consts,
    models::{
        response::ResponseBody,
        user::{LoginDTO, UserDTO},
    },
    services::user_service,
};
use crate::models::user::{UserInfoDTO};

#[post("/api/auth/signup")]
pub async fn signup(user_dto: Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(
            HttpResponse::Ok().json(ResponseBody::new(200, &message, consts::EMPTY))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/auth/login")]
pub async fn login(login_dto: Json<LoginDTO>, id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::login(login_dto.0, id, &pool) {
        Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            200,
            consts::MESSAGE_LOGIN_SUCCESS,
            res,
        ))),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/auth/logout")]
pub async fn logout(id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::logout(id, &pool) {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            200,
            consts::MESSAGE_LOGOUT_SUCCESS,
            consts::EMPTY,
        ))),
        Err(_) => Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            400,
            consts::MESSAGE_INVALID_AUTH_DATA,
            consts::EMPTY,
        )))
    }
}

#[get("/api/auth/info")]
pub async fn info(id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::get_user_by_identity(id, &pool.get().unwrap()) {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            200,
            consts::MESSAGE_OK,
            UserInfoDTO {
                id: user.id,
                username: user.username,
                email: user.email,
                is_admin: user.is_admin,
                created_at: user.created_at,
            },
        ))),
        Err(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            200,
            consts::MESSAGE_INVALID_AUTH_DATA,
            consts::EMPTY,
        ))),
    }
}