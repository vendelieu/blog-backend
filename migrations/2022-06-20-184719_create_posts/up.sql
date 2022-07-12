create table posts
(
    id                serial primary key,
    image             varchar                     not null,
    title             varchar                     not null,
    content           text                        not null,
    short_content     varchar(140) generated always as (substring(content, 1, 140)) stored,
    slug              varchar                     not null unique,
    commentaries_open bool                        not null default true,
    updated_at        timestamp without time zone not null default now(),
    created_at        timestamp without time zone not null default now()
);