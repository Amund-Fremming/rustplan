-- Add migration script here
CREATE TABLE member (
    id SERIAL PRIMARY KEY,
    group_id INTEGER NOT NULL,
    locked_reply BOOLEAN NOT NULL
);
