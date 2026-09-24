CREATE TABLE org (
    org_id  INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name    TEXT NOT NULL
);

CREATE TABLE person (
    person_id  INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    org_id     INTEGER NOT NULL REFERENCES org,
    name       TEXT NOT NULL
);

CREATE TABLE ticket_status (
    status_id  INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    org_id     INTEGER NOT NULL REFERENCES org ON DELETE CASCADE,
    name       TEXT NOT NULL,
    UNIQUE (org_id, name)
);

CREATE TABLE ticket (
    ticket_id    INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    status_id    INTEGER NOT NULL REFERENCES ticket_status,
    priority     SMALLINT NOT NULL CHECK (priority BETWEEN 0 AND 255),
    title        TEXT NOT NULL,
    raised_by    INTEGER NOT NULL REFERENCES person,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at    TIMESTAMPTZ,
    description  TEXT
);
