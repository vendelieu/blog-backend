create table tags
(
    id   serial primary key,
    name varchar not null,
    slug varchar not null unique
);