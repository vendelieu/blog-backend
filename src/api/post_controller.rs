use actix_web::{HttpResponse, Result, web};

use crate::{
    configurations::db::Pool,
    consts,
    models::{filters::PostFilter, response::ResponseBody},
    services::post_service,
};

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
            ResponseBody::new(200, consts::MESSAGE_OK, post))
        ),
        Err(err) => Ok(err.response()),
    }
}

#[get("/api/post/{slug}/related")]
pub async fn find_related(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match post_service::get_related(id.into_inner(), &pool) {
        Ok(post) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(200, consts::MESSAGE_OK, post))
        ),
        Err(err) => Ok(err.response()),
    }
}
