use actix_web::web::Data;
use chrono::{Duration, TimeZone, Utc};
use sitewriter::{ChangeFreq, UrlEntry, UrlEntryBuilder};

use crate::configurations::db::Pool;
use crate::models::filters::PostFilter;
use crate::services::post_service;

pub async fn get_sitemap(pool: &Data<Pool>) -> String {
    let posts = post_service::filter(PostFilter {
        title: None,
        keyword: None,
        tags: None,
        slug: None,
        date: None,
        sort_by: None,
        page: None,
        page_size: Some(i64::from(1000)),
    }, pool).unwrap().data;

    let urls = vec![
        UrlEntryBuilder::default()
            .loc("https://vendeli.eu/sitemap.xml".parse().unwrap())
            .build()
            .unwrap(),
        UrlEntry {
            loc: "https://vendeli.eu/".parse().unwrap(),
            changefreq: Some(ChangeFreq::Always),
            priority: Some(1.0),
            lastmod: Some(Utc::now()),
        },
        UrlEntry {
            loc: "https://vendeli.eu/projects".parse().unwrap(),
            changefreq: Some(ChangeFreq::Yearly),
            priority: Some(0.4),
            lastmod: Some(Utc::now() - Duration::days(150)),
        },
        UrlEntry {
            loc: "https://vendeli.eu/about".parse().unwrap(),
            changefreq: Some(ChangeFreq::Never),
            priority: Some(0.4),
            lastmod: Some(Utc::now() - Duration::days(300)),
        },
    ];

    let posts_entries: Vec<UrlEntry> = posts.into_iter().map(|i|
        UrlEntry {
            loc: format!("https://vendeli.eu/post/{}", i.slug).parse().unwrap(),
            changefreq: Some(ChangeFreq::Monthly),
            priority: Some(0.7),
            lastmod: Some(Utc.from_utc_datetime(&i.updated_at)),
        }
    ).collect();

    sitewriter::generate_str(&[urls, posts_entries].concat())
}
