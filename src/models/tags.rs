use diesel::QueryResult;
use crate::{
    configurations::db::Connection,
    schema::tags::{self, dsl::*},
};
use diesel::prelude::*;
use crate::schema::post_tags_pivot::dsl::post_tags_pivot;
use crate::schema::post_tags_pivot::{post_slug, tag_slug};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "tags"]
pub struct TagDTO {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostTagsPivot {
    pub id: i32,
    pub post_slug: String,
    pub tag_slug: String,
}

impl Tag {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Tag>> {
        tags.order(id.asc()).load::<Tag>(conn)
    }
    
    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Tag> {
        tags.find(i).get_result::<Tag>(conn)
    }

    pub fn find_by_name(s: &str, conn: &Connection) -> QueryResult<Vec<Tag>> {
        tags.filter(name.ilike(format!("%{}%", s))).load::<Tag>(conn)
    }

    pub fn find_by_post_slug(s: &str, conn: &Connection) -> QueryResult<Vec<Tag>> {
        tags.filter(
            slug.eq_any(
                post_tags_pivot.select(tag_slug).filter(post_slug.eq(s))
            )
        ).load::<Tag>(conn)
    }

    pub fn insert(new_tag: TagDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(tags)
            .values(&new_tag)
            .execute(conn)
    }

    pub fn update(i: i32, updated_tag: TagDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(tags.find(i))
            .set(&updated_tag)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(tags.find(i)).execute(conn)
    }
}