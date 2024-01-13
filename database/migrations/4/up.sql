CREATE TABLE IF NOT EXISTS activations (
    id          INT8 PRIMARY KEY,
    code        BYTEA UNIQUE,
    data        BYTEA
);