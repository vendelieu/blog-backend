use actix_identity::Identity;
use crate::{
    configurations::db::Pool,
    consts,
    models::{filters::CommentaryFilter, commentaries::CommentaryDTO, response::ResponseBody},
};
use actix_web::{web, HttpResponse, Result};
use crate::services::commentaries_service;

#[get("/api/post/{slug}/commentaries")]
pub async fn find_by_slug(
    slug: web::Path<String>,
    web::Query(filter): web::Query<CommentaryFilter>,
    pool: web::Data<Pool>) -> Result<HttpResponse>
{
    match commentaries_service::filter_by_post_slug(slug.into_inner(), filter, &pool) {
        Ok(comments) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(consts::MESSAGE_OK, comments))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[post("/api/commentary/{id}")]
pub async fn insert(
    p_id: web::Path<i32>,
    new_comment: web::Json<CommentaryDTO>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match commentaries_service::insert(p_id.into_inner(), new_comment.0, id, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(consts::MESSAGE_OK, consts::EMPTY))),
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
                ResponseBody::new(consts::MESSAGE_OK, consts::EMPTY))
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
                ResponseBody::new(consts::MESSAGE_OK, consts::EMPTY))
            )
        }
        Err(err) => Ok(err.response()),
    }
}