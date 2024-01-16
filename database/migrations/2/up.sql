CREATE TABLE IF NOT EXISTS invites (
    id          SERIAL8 PRIMARY KEY,
    person_id   INT8 NOT NULL,
    public_key  BYTEA UNIQUE,
    secure_key  BYTEA UNIQUE,
    data        BYTEA
);
CREATE INDEX IF NOT EXISTS invites_person_id_idx ON invites(person_id);