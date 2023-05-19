use crate::api::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg
        //main
        .service(main_controller::index)
        // posts
        .service(post_controller::insert)
        .service(post_controller::find_by_slug)
        .service(post_controller::find_related)
        .service(post_controller::update)
        .service(post_controller::delete)
        .service(post_controller::get_posts)
        // tags
        .service(tag_controller::find_all)
        .service(tag_controller::find_by_slug)
        .service(tag_controller::find_by_post_slug)
        .service(tag_controller::insert)
        .service(tag_controller::update)
        .service(tag_controller::delete)
    ;
}