-- Add migration script here
-- migrations/YYYYMMDDHHMMSS_create_tables.sql
CREATE TABLE taglists (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tag_items (
    id UUID PRIMARY KEY,
    taglist_id UUID NOT NULL REFERENCES taglists(id) ON DELETE CASCADE,
    tag VARCHAR(255) NOT NULL,
    remark TEXT
);