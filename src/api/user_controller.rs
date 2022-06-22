use actix_identity::Identity;
use crate::{
    configurations::db::Pool,
    consts,
    models::{
        response::ResponseBody,
        user::{LoginDTO, UserDTO},
    },
    services::user_service,
};
use actix_web::{web, HttpResponse, Result};
use web::Json;

#[post("/api/auth/signup")]
pub async fn signup(user_dto: Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(
            HttpResponse::Ok().json(ResponseBody::new(&message, consts::EMPTY))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/auth/login")]
pub async fn login(login_dto: Json<LoginDTO>, id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::login(login_dto.0, id, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            consts::MESSAGE_LOGIN_SUCCESS,
            token_res,
        ))),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/auth/logout")]
pub async fn logout(id: Identity, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::logout(id, &pool) {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            consts::MESSAGE_LOGOUT_SUCCESS,
            consts::EMPTY,
        ))),
        Err(_) => Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            consts::MESSAGE_INVALID_AUTH_DATA,
            consts::EMPTY,
        )))
    }
}