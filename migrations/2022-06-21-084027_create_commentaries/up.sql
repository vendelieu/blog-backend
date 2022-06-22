create table commentaries
(
    id         serial primary key,
    post_id    int                         not null references posts (id),
    user_id    int                         not null references users (id),
    text       varchar(2048)               not null,
    reply_to   int,
    created_at timestamp without time zone not null default now(),
    updated_at timestamp without time zone not null default now()
);