-- Add migration script here
CREATE TABLE IF NOT EXISTS stories (
    id bigserial PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    author_id INTEGER NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_stories_author_id ON stories(author_id);
CREATE INDEX IF NOT EXISTS idx_stories_author_id_created_at ON stories(author_id, created_at);
CREATE INDEX IF NOT EXISTS idx_stories_author_id_updated_at ON stories(author_id, updated_at);