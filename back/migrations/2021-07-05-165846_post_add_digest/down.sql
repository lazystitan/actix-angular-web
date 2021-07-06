-- This file should undo anything in `up.sql`
alter table posts drop column digest, drop column viewers_number, drop column like_number, drop column dislike_number;