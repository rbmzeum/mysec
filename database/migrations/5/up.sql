CREATE TABLE IF NOT EXISTS activation_person (
    activation_id   INT8 NOT NULL,
    person_id       INT8 NOT NULL,
    data            BYTEA,
    FOREIGN KEY(activation_id) REFERENCES activations(id),
    FOREIGN KEY(person_id) REFERENCES persons(id),
    PRIMARY KEY (activation_id, person_id)
);