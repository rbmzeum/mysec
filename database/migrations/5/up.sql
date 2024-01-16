CREATE TABLE IF NOT EXISTS activation_person (
    activation_id   INT8 NOT NULL,
    person_id       INT8 NOT NULL,
    data            BYTEA,
    FOREIGN KEY(activation_id) REFERENCES activations(id),
    PRIMARY KEY (activation_id, person_id)
);
CREATE INDEX IF NOT EXISTS activation_person_person_id_idx ON activation_person(person_id);