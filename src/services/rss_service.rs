use std::env;

use actix_web::web;
use rss::{ChannelBuilder, Item};
use web::Data;

use crate::{
    configurations::db::Pool,
    models::filters::PostFilter,
    services::post_service,
};

pub async fn get_feed(filter: PostFilter, pool: &Data<Pool>) -> String {
    let mut channel = ChannelBuilder::default()
        .title(env::var("BLOG_TITLE").unwrap())
        .link(env::var("BLOG_URL").unwrap())
        .description(env::var("BLOG_DESCRIPTION").unwrap())
        .build();

    let posts = post_service::filter(filter, pool).unwrap().data;
    let items: Vec<Item> = posts.into_iter().map(|x| x.into()).collect();
    channel.set_items(items);

    channel.to_string()
}