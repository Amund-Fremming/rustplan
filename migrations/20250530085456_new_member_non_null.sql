-- Add migration script here
CREATE TABLE member (
    id SERIAL PRIMARY KEY,
    group_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL DEFAULT '',
    locked_reply BOOLEAN NOT NULL DEFAULT false
);