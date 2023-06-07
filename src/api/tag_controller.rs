use actix_web::{HttpResponse, Result, web};

use crate::{
    configurations::db::Pool,
    consts,
    models::response::ResponseBody,
    services::tags_service,
};

#[get("/api/tags")]
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match tags_service::find_all(&pool) {
        Ok(tags) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(200, consts::MESSAGE_OK, tags))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[get("/api/tag/{name}")]
pub async fn find_by_name(name: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match tags_service::find_by_name(name.into_inner(), &pool) {
        Ok(tag) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(200, consts::MESSAGE_OK, tag))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[get("/api/post/{slug}/tags")]
pub async fn find_by_post_slug(slug: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match tags_service::find_by_post_slug(slug.into_inner(), &pool) {
        Ok(tags) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(200, consts::MESSAGE_OK, tags))
        ),
        Err(err) => Ok(err.response()),
    }
}
