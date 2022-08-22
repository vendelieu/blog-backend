use actix_cors::Cors;
use actix_web::http;
use http::header::{ACCEPT, CONTENT_TYPE};

pub fn get_config() -> Cors {
    Cors::default() // allowed_origin return access-control-allow-origin: * by default
        .allowed_origin("http://127.0.0.1:4200")
        .allowed_origin("http://localhost:4200")
        .allowed_origin("https://vendeli.eu")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![ACCEPT, CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}