use actix_web::{HttpResponse, Result, web};

use crate::configurations::db::Pool;
use crate::consts;
use crate::models::response::ResponseBody;
use crate::models::tags::TagDTO;
use crate::services::tags_service;

#[post("/api/admin/tag")]
pub async fn insert(
    new_tag: web::Json<TagDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match tags_service::insert(new_tag.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[put("/api/admin/tag/{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_tag: web::Json<TagDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match tags_service::update(id.into_inner(), updated_tag.0, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}

#[delete("/api/admin/tag/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match tags_service::delete(id.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}