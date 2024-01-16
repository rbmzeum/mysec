CREATE TABLE IF NOT EXISTS persons (
    id      INT8 NOT NULL,
    version INT8 NOT NULL,
    data    BYTEA,
    PRIMARY KEY (id, version)
);
CREATE TABLE IF NOT EXISTS deletes_persons (
    person_id   INT8 PRIMARY KEY,
    data        BYTEA
);