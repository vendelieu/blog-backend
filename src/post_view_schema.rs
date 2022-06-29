#![allow(unused_imports)]
table! {
    use diesel::sql_types::*;
    use diesel::prelude::*;
    use crate::utils::db_tag_type_wrapper::TagType;
    use crate::utils::db_nav_post_type_wrapper::NavPostType;

    post_view(id) {
        id -> Int4,
        title -> Varchar,
        short_content -> Varchar,
        slug -> Varchar,
        commentaries_open -> Bool,
        tags -> Nullable<Array<TagType>>,
        prev -> Nullable<NavPostType>,
        next -> Nullable<NavPostType>,
        updated_at -> Timestamp,
    }
}

use crate::schema::post_tags_pivot;
allow_tables_to_appear_in_same_query!(
    post_tags_pivot,
    post_view
);