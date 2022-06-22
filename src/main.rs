#![allow(unused_must_use)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
extern crate validator;
extern crate serde_json;
extern crate actix_cors;
extern crate actix_rt;
extern crate bcrypt;
extern crate derive_more;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate serde;
extern crate uuid;

use std::{env, io};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{HttpServer, App, http, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;


mod consts;
mod schema;

mod api;
mod configurations;
mod models;
mod services;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    }
    env_logger::init();

    let connection_url = env::var("DATABASE_URL").expect("Database Url");
    let domain = env::var("DOMAIN").expect("DOMAIN is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let bind_address = format!("{}:{}", domain, port);

    let pool = configurations::db::migrate_and_config(&connection_url);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&consts::SECRET_KEY)
                        .name("auth")
                        .path("/")
                        .domain(domain.as_str())
                        .max_age_secs(chrono::Duration::days(1).num_seconds())
                        .secure(false), // this can only be true if you have https
                )
            )
            .app_data(web::JsonConfig::default().limit(4096))
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(api::main_controller::index)
            .configure(configurations::router::configure)
    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
        .await
}