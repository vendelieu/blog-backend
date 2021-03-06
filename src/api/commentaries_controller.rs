use actix_identity::Identity;
use actix_web::{HttpResponse, Result, web};

use crate::{
    configurations::db::Pool,
    consts,
    models::{commentaries::CommentaryDTO, filters::CommentaryFilter, response::ResponseBody},
};
use crate::services::commentaries_service;

#[get("/api/post/{slug}/commentaries")]
pub async fn find_by_slug(
    slug: web::Path<String>,
    web::Query(filter): web::Query<CommentaryFilter>,
    pool: web::Data<Pool>) -> Result<HttpResponse>
{
    match commentaries_service::filter_by_post_slug(slug.into_inner(), filter, &pool) {
        Ok(comments) => Ok(HttpResponse::Ok().json(comments)),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/post/{slug}/commentary")]
pub async fn insert(
    p_slug: web::Path<String>,
    new_comment: web::Json<CommentaryDTO>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match commentaries_service::insert(p_slug.into_inner(), new_comment.0, id, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[put("/api/commentary/{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_post: web::Json<CommentaryDTO>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match commentaries_service::update(
        id.into_inner(), updated_post.0, identity, &pool,
    ) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}

#[delete("/api/commentary/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match commentaries_service::delete(id.into_inner(), identity, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(
                ResponseBody::new(200, consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}