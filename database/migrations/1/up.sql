CREATE TABLE IF NOT EXISTS persons (
    id      SERIAL PRIMARY KEY,
    name    TEXT NOT NULL,
    data    BYTEA
);