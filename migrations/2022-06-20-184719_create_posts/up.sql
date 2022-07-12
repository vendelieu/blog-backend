create table posts
(
    id                serial primary key,
    image             varchar                     not null,
    title             varchar                     not null,
    content           text                        not null,
    description       varchar(140)                not null,
    slug              varchar                     not null unique,
    commentaries_open bool                        not null default true,
    updated_at        timestamp without time zone not null default now(),
    created_at        timestamp without time zone not null default now()
);