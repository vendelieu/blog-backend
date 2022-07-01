create table commentaries
(
    id         serial primary key,
    post_slug  varchar                     not null references posts (slug),
    username   varchar                     not null references users (username),
    text       varchar(400)                not null,
    reply_to   int references commentaries (id) on DELETE set null on UPDATE cascade,
    created_at timestamp without time zone not null default now(),
    updated_at timestamp without time zone not null default now()
);