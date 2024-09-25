-- Add migration script here
ALTER TABLE
    stories
ALTER COLUMN
    author_id TYPE BIGINT;