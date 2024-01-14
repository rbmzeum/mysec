CREATE TABLE IF NOT EXISTS invite_shared_keys (
    invite_id   SERIAL8 PRIMARY KEY,
    shared_key  BYTEA UNIQUE,
    data        BYTEA
);