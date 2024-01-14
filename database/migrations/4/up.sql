CREATE TABLE IF NOT EXISTS activations (
    id          SERIAL8 PRIMARY KEY,
    code        BYTEA UNIQUE,
    data        BYTEA
);