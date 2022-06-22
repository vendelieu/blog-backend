create table posts
(
    id           serial primary key,
    title        varchar                     not null,
    content      text                        not null,
    content_html text                        not null,
    slug         varchar                     not null unique,
    updated_at   timestamp without time zone not null default now(),
    created_at   timestamp without time zone not null default now()
);