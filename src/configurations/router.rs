use crate::api::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg
        // user
        .service(user_controller::signup)
        .service(user_controller::login)
        .service(user_controller::logout)
        // posts
        .service(post_controller::find_all)
        .service(post_controller::insert)
        .service(post_controller::find_by_slug)
        .service(post_controller::update)
        .service(post_controller::delete)
        .service(post_controller::filter)
        // tags
        .service(tag_controller::find_all)
        .service(tag_controller::find_by_slug)
        .service(tag_controller::find_by_post_slug)
        .service(tag_controller::insert)
        .service(tag_controller::update)
        .service(tag_controller::delete)
        // comments
        .service(commentaries_controller::find_by_slug)
        .service(commentaries_controller::insert)
        .service(commentaries_controller::update)
        .service(commentaries_controller::delete)
    ;
}