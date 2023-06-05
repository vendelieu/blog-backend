use actix_web::{HttpResponse, Result, web};

use crate::{
    configurations::db::Pool,
    consts,
    models::{post::PostDTO, response::ResponseBody},
    services::post_service,
};

#[post("/api/admin/post")]
pub async fn insert(
    new_post: web::Json<PostDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::insert(new_post.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/admin/post/{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_post: web::Json<PostDTO>,
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

#[post("/api/admin/post/delete/{id}")]
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