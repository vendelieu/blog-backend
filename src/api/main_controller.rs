use actix_web::{get, HttpResponse, web};
use web::Query;

use crate::{
    configurations::db::Pool,
    models::filters::PostFilter,
    services::rss_service,
};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[get("/rss")]
pub async fn rss(
    Query(filter): Query<PostFilter>,
    pool: web::Data<Pool>,
) -> HttpResponse {
    HttpResponse::Ok().body(rss_service::get_feed(filter, &pool).await)
}

#[get("/api/admin/check")]
pub async fn check_admin_status() -> HttpResponse {
    HttpResponse::Ok().finish()
}