CREATE TABLE IF NOT EXISTS trust_chain (
    id          BYTEA PRIMARY KEY,
    parent_id   BYTEA UNIQUE,
    data        BYTEA
);