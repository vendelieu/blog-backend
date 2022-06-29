table! {
    commentaries (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        text -> Varchar,
        reply_to -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    post_tags_pivot (id) {
        id -> Int4,
        post_slug -> Varchar,
        tag_slug -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        short_content -> Nullable<Varchar>,
        slug -> Varchar,
        commentaries_open -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Nullable<Varchar>,
        is_admin -> Bool,
        created_at -> Timestamp,
    }
}

joinable!(commentaries -> posts (post_id));
joinable!(commentaries -> users (user_id));

allow_tables_to_appear_in_same_query!(
    commentaries,
    post_tags_pivot,
    posts,
    tags,
    users,
);
