use std::env;

use actix_web::cookie::time::format_description::well_known::Rfc2822;
use actix_web::cookie::time::OffsetDateTime;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use rss::{Guid, Item};
use validator::Validate;

use crate::{
    configurations::db::Connection,
    models::pagination::SortingAndPaging,
    schema::post_tags_pivot::{dsl::post_tags_pivot, post_slug, tag_slug},
};
use crate::post_view_schema::post_view::{self as p_view, dsl::post_view, slug};
use crate::utils::db_nav_post_type_wrapper::NavPost;
use crate::utils::db_tag_type_wrapper::Tag;

use super::{filters::{PostFilter, Sort}, response::Page};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub image: String,
    pub title: String,
    pub content: String,
    pub description: String,
    pub slug: String,
    pub commentaries_open: bool,
    pub tags: Option<Vec<Tag>>,
    pub prev: Option<NavPost>,
    pub next: Option<NavPost>,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Validate)]
#[table_name = "p_view"]
pub struct PostDTO {
    #[validate(url)]
    pub image: String,
    #[validate(length(min = 2, max = 128))]
    pub title: String,
    #[validate(length(min = 2))]
    pub content: String,
    #[validate(length(min = 2, max = 256))]
    pub description: String,
    pub commentaries_open: Option<bool>,
    #[validate(length(min = 2, max = 64))]
    pub slug: String,
}

impl Post {
    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Post> {
        post_view.find(i).get_result::<Post>(conn)
    }

    pub fn find_by_slug(s: &str, conn: &Connection) -> QueryResult<Post> {
        post_view.filter(slug.eq(s)).get_result::<Post>(conn)
    }

    pub fn filter(filter: PostFilter, conn: &Connection) -> QueryResult<Page<Post>> {
        let mut query = p_view::table.into_boxed();

        if let Some(i) = filter.title {
            query = query.filter(p_view::title.like(format!("%{}%", i)));
        }
        if let Some(i) = filter.keyword {
            query = query.filter(p_view::content.like(format!("%{}%", i)));
        }
        if let Some(i) = filter.slug {
            query = query.filter(slug.like(format!("%{}%", i)));
        }
        if let Some(i) = filter.tags {
            let split_tags = i.split(',').map(|s| s.to_string()).collect::<Vec<String>>();

            query = query.filter(
                slug.eq_any(
                    post_tags_pivot.select(post_slug)
                        .filter(tag_slug.eq_any(split_tags))
                )
            );
        }
        if let Some(i) = filter.date {
            query = query.filter(p_view::updated_at.ge(i));
        }

        let sort = match filter.sort_by {
            Some(sort_type) => match sort_type {
                Sort::Newest => "DESC",
                Sort::Oldest => "ASC"
            }
            None => "DESC"
        }.to_string();

        query
            .paginate(
                filter
                    .page
                    .unwrap_or(crate::consts::DEFAULT_PAGE_NUM),
            )
            .per_page(
                filter
                    .page_size
                    .unwrap_or(crate::consts::DEFAULT_PER_PAGE),
            )
            .sort(
                "updated_at".to_string(),
                sort,
            )
            .load_and_count_items::<Post>(conn)
    }

    pub fn filter_related(s: String, conn: &Connection) -> QueryResult<Vec<NavPost>> {
        sql_query("select title, slug from posts p left join post_tags_pivot ptp on p.slug = ptp.post_slug where ptp.tag_slug in (select tag_slug from post_tags_pivot where post_slug = $1) and p.slug != $1 limit 5")
            .bind::<Text, _>(s)
            .get_results::<NavPost>(conn)
    }

    pub fn insert(new_post: PostDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(post_view)
            .values(&new_post)
            .execute(conn)
    }

    pub fn update(i: i32, updated_post: PostDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(post_view.find(i))
            .set(&updated_post)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(post_view.find(i)).execute(conn)
    }
}

impl From<Post> for Item {
    fn from(value: Post) -> Self {
        let dt = OffsetDateTime::from_unix_timestamp(
            value.updated_at.timestamp()
        ).unwrap().format(&Rfc2822).unwrap();

        let mut item = Item::default();
        item.set_title(value.title.clone());
        item.set_pub_date(dt);
        item.set_description(value.description);
        item.set_link(env::var("BLOG_URL").unwrap() + "/" + value.slug.as_str());
        // unique id for each post across the site
        let mut guid = Guid::default();
        guid.set_value(&value.slug);
        item.set_guid(guid);

        item
    }
}
