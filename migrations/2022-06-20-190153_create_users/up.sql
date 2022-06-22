CREATE TABLE users
(
    id            SERIAL PRIMARY KEY          NOT NULL,
    username      VARCHAR                     NOT NULL UNIQUE,
    email         VARCHAR                     NOT NULL UNIQUE,
    password      VARCHAR                     NOT NULL,
    login_session VARCHAR,
    is_admin      bool                        not null default false,
    created_at    timestamp without time zone not null default now()
);