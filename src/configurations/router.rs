use actix_web::web;

use crate::api::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg
        //main
        .service(main_controller::index)
        .service(main_controller::rss)
        .service(main_controller::sitemap)
        // posts
        .service(post_controller::find_by_slug)
        .service(post_controller::find_related)
        .service(post_controller::get_posts)
        // tags
        .service(tag_controller::find_all)
        .service(tag_controller::find_by_name)
        .service(tag_controller::find_by_post_slug)

        // admin
        .service(main_controller::check_admin_status)
        .service(tag_admin_controller::delete)
        .service(tag_admin_controller::insert)
        .service(tag_admin_controller::link)
        .service(tag_admin_controller::unlink)
        .service(tag_admin_controller::update)
        .service(tag_admin_controller::delete)
        .service(post_admin_controller::insert)
        .service(post_admin_controller::update)
        .service(post_admin_controller::delete)
    ;
}