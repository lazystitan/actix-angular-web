-- Your SQL goes here
create table token_history (
    id SERIAL PRIMARY KEY,
    token VARCHAR NOT NULL,
    create_time timestamp not null default current_timestamp,
    expire_time timestamp not null
)