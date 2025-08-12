create table files
(
    id            text primary key,
    short_id      text unique not null,
    original_name text,
    mime_type     text,
    size_bytes    bigint      not null,
    path          text        not null,
    max_downloads int                  default null,
    downloads     int                  default 0,
    created_at    datetime    not null default (current_timestamp),
    expires_at    datetime,
    deleted       boolean              default false
)