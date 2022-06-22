create table post_tags_pivot
(
    id        serial primary key,
    post_slug varchar not null references posts (slug),
    tag_slug  varchar not null references Tags (slug),
    unique (post_slug, tag_slug)
);