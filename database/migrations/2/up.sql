CREATE TABLE IF NOT EXISTS clients (
    id      SERIAL PRIMARY KEY,
    name    TEXT NOT NULL,
    data    BYTEA
);