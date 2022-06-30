use actix_identity::Identity;
use crate::{
    configurations::db::Pool,
    consts,
    models::{tags::TagDTO, response::ResponseBody},
    services::tags_service,
};
use actix_web::{web, HttpResponse, Result};

#[get("/api/tags")]
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match tags_service::find_all(&pool) {
        Ok(tags) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(200, consts::MESSAGE_OK, tags))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[get("/api/tag/{slug}")]
pub async fn find_by_slug(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match tags_service::find_by_slug(id.into_inner(), &pool) {
        Ok(tag) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(200, consts::MESSAGE_OK, tag))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[get("/api/post/{slug}/tags")]
pub async fn find_by_post_slug(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match tags_service::find_by_post_slug(id.into_inner(), &pool) {
        Ok(tags) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(200, consts::MESSAGE_OK, tags))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/tag")]
pub async fn insert(
    new_tag: web::Json<TagDTO>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match tags_service::insert(new_tag.0, id, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[put("/api/tag/{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_tag: web::Json<TagDTO>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match tags_service::update(id.into_inner(), updated_tag.0, identity, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}

#[delete("/api/tag/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match tags_service::delete(id.into_inner(), identity, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}