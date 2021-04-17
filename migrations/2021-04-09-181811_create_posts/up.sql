CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f',
    create_time timestamp not null default current_timestamp,
    last_update_time timestamp not null default current_timestamp
)