CREATE TABLE IF NOT EXISTS activations (
    id          SERIAL8 PRIMARY KEY,
    code        BYTEA UNIQUE,
    data        BYTEA
);
CREATE TABLE IF NOT EXISTS disables_activations (
    activation_id   INT8 PRIMARY KEY,
    data            BYTEA
);