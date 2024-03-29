#![allow(unused_must_use)]

extern crate actix_cors;
extern crate actix_rt;
#[macro_use]
extern crate actix_web;
extern crate derive_more;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
extern crate validator;

use std::{env, io};

use actix_governor::Governor;
use actix_ip_filter::IPFilter;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use configurations::rate_limiting_governor;

mod consts;
mod schema;
mod post_view_schema;

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
    let ssl_cert = env::var("CERT_PATH").expect("CERT_PATH is not set");
    let ssl_key = env::var("CERT_KEY_PATH").expect("CERT_KEY_PATH is not set");

    let pool = configurations::db::migrate_and_config(&connection_url);

    let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    builder.set_private_key_file(ssl_key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(ssl_cert).unwrap();


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // enable rate-limiting middleware
            .wrap(Governor::new(&rate_limiting_governor::get_governor()))
            .wrap(IPFilter::new()
                .allow(vec![env::var("ADMIN_IP").unwrap().as_str()])
                .limit_to(vec!["/api/admin/*"])
            )
            .app_data(Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(4096))
            .wrap(configurations::cors::get_config())
            .configure(configurations::router::configure)
    })
        .bind_openssl(&bind_address, builder)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
        .await
}