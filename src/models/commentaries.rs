use chrono::NaiveDateTime;


use crate::{
    configurations::db::Connection,
    models::pagination::SortingAndPaging,
    schema::commentaries::{self, dsl::*},
};
use diesel::prelude::*;

use crate::models::filters::{CommentaryFilter, Sort};
use crate::models::response::Page;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Commentary {
    pub id: i32,
    pub post_slug: String,
    pub username: String,
    pub text: String,
    pub reply_to: Option<i32>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CommentaryDTO {
    pub text: String,
    pub reply_to: Option<i32>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "commentaries"]
pub struct CommentaryFullDTO {
    pub post_slug: String,
    pub username: String,
    pub text: String,
    pub reply_to: Option<i32>,
}

impl Commentary {
    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Commentary> {
        commentaries.find(i).get_result::<Commentary>(conn)
    }

    pub fn filter_by_post_slug(s: &str, filter: CommentaryFilter, conn: &Connection) -> QueryResult<Page<Commentary>> {
        let mut query = commentaries::table.into_boxed();


        query = query.filter(post_slug.eq(s));

        if let Some(i) = filter.username {
            query = query.filter(username.eq(i));
        }
        if let Some(i) = filter.text {
            query = query.filter(text.like(format!("%{}%", i)));
        }
        if let Some(i) = filter.reply_to {
            query = query.filter(reply_to.eq(i));
        }
        if let Some(i) = filter.date {
            query = query.filter(commentaries::updated_at.ge(i));
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
                "created_at".to_string(),
                sort,
            )
            .load_and_count_items::<Commentary>(conn)
    }

    pub fn insert(new_comment: CommentaryFullDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(commentaries)
            .values(&new_comment)
            .execute(conn)
    }

    pub fn update(i: i32, updated_comment: CommentaryFullDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(commentaries.find(i))
            .set(&updated_comment)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(commentaries.find(i)).execute(conn)
    }
}