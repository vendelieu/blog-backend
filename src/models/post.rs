use chrono::NaiveDateTime;
use crate::{
    configurations::db::Connection,
    models::pagination::SortingAndPaging,
    schema::posts::{self, dsl::*},
    schema::post_tags_pivot::{dsl::post_tags_pivot, post_slug, tag_slug},
};
use diesel::prelude::*;
use super::{filters::PostFilter, response::Page};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub content_html: String,
    pub slug: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct PostDTO {
    pub title: String,
    pub content: String,
    pub content_html: String,
    pub slug: String,
}

impl Post {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Post>> {
        posts.order(id.asc()).load::<Post>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Post> {
        posts.find(i).get_result::<Post>(conn)
    }

    pub fn find_by_slug(s: &str, conn: &Connection) -> QueryResult<Post> {
        posts.filter(slug.eq(s)).get_result::<Post>(conn)
    }

    pub fn filter(filter: PostFilter, conn: &Connection) -> QueryResult<Page<Post>> {
        let mut query = posts::table.into_boxed();

        if let Some(i) = filter.title {
            query = query.filter(title.like(format!("%{}%", i)));
        }
        if let Some(i) = filter.text {
            query = query.filter(content.like(format!("%{}%", i)));
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
            query = query.filter(updated_at.ge(i));
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

    pub fn insert(new_post: PostDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(posts)
            .values(&new_post)
            .execute(conn)
    }

    pub fn update(i: i32, updated_post: PostDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(posts.find(i))
            .set(&updated_post)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(posts.find(i)).execute(conn)
    }
}
