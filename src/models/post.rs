use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;

use crate::{
    configurations::db::Connection,
    models::pagination::SortingAndPaging,
    schema::post_tags_pivot::{dsl::post_tags_pivot, post_slug, tag_slug},
};
use crate::post_view_schema::post_view::{self as p_view, dsl::post_view, slug};
use crate::utils::db_nav_post_type_wrapper::NavPost;
use crate::utils::db_tag_type_wrapper::Tag;

use super::{filters::PostFilter, response::Page};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub short_content: String,
    pub slug: String,
    pub commentaries_open: bool,
    pub tags: Option<Vec<Tag>>,
    pub prev: Option<NavPost>,
    pub next: Option<NavPost>,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "p_view"]
pub struct PostDTO {
    pub title: String,
    pub content: String,
    pub commentaries_open: bool,
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

        query
            .paginate(
                filter
                    .page_num
                    .unwrap_or(crate::consts::DEFAULT_PAGE_NUM),
            )
            .per_page(
                filter
                    .page_size
                    .unwrap_or(crate::consts::DEFAULT_PER_PAGE),
            )
            .sort(
                filter
                    .sort_by
                    .unwrap_or_else(|| crate::consts::EMPTY_STR.to_string()),
                filter
                    .sort_direction
                    .unwrap_or_else(|| crate::consts::EMPTY_STR.to_string()),
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
