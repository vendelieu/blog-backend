use actix_web::{HttpResponse, Result, web};
use actix_web_validator::Json;

use crate::{
    configurations::db::Pool,
    consts,
    models::{post::PostDTO, response::ResponseBody},
    services::post_service,
};

#[post("/api/admin/post")]
pub async fn insert(
    new_post: Json<PostDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::insert(new_post.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[put("/api/admin/post/{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_post: Json<PostDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::update(id.into_inner(), updated_post.0, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}

#[delete("/api/admin/post/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::delete(id.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}