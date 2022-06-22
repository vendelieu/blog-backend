use actix_cors::Cors;
use actix_web::http;

pub fn get_config() -> Cors {
    Cors::default() // allowed_origin return access-control-allow-origin: * by default
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_origin("http://localhost:3000")
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}