use actix_identity::Identity;
use crate::{
    configurations::db::Pool,
    consts,
    models::{filters::PostFilter, post::PostDTO, response::ResponseBody},
    services::post_service,
};
use actix_web::{web, HttpResponse, Result};

#[get("/api/posts")]
pub async fn get_posts(
    web::Query(filter): web::Query<PostFilter>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::filter(filter, &pool) {
        Ok(page) => Ok(HttpResponse::Ok().json(page)),
        Err(err) => Ok(err.response()),
    }
}

#[get("/api/post/{slug}")]
pub async fn find_by_slug(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match post_service::find_by_slug(id.into_inner(), &pool) {
        Ok(post) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(consts::MESSAGE_OK, post))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/post")]
pub async fn insert(
    new_post: web::Json<PostDTO>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::insert(new_post.0, id, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(consts::MESSAGE_OK, consts::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[put("/api/post/{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_post: web::Json<PostDTO>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::update(id.into_inner(), updated_post.0, identity, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}

#[delete("/api/post/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match post_service::delete(id.into_inner(), identity, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}