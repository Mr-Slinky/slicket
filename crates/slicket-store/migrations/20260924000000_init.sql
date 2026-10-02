CREATE TABLE org
(
    org_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name   TEXT NOT NULL
);

CREATE TABLE tenant
(
    tenant_id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (tenant_id),
    org_id    INTEGER NOT NULL REFERENCES org
);

CREATE TABLE person
(
    person_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    org_id    INTEGER NOT NULL REFERENCES org,
    name      TEXT    NOT NULL,
    email     TEXT    NOT NULL CHECK (email = lower(email))
);

CREATE TABLE ticket_type
(
    ticket_type_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name           TEXT NOT NULL,
    UNIQUE (name)
);

CREATE TABLE ticket_status
(
    status_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name      TEXT NOT NULL,
    UNIQUE (name)
);

CREATE TABLE ticket
(
    ticket_id      INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    ticket_type_id INTEGER     NOT NULL REFERENCES ticket_type,
    status_id      INTEGER     NOT NULL REFERENCES ticket_status,
    priority       SMALLINT    NOT NULL CHECK (priority BETWEEN 0 AND 255),
    title          TEXT        NOT NULL,
    raised_by      INTEGER     NOT NULL REFERENCES person,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at      TIMESTAMPTZ,
    description    TEXT,
    CHECK (closed_at IS NULL OR closed_at >= created_at)
);

CREATE INDEX person_org_id_idx    ON person (org_id);
CREATE INDEX ticket_status_id_idx ON ticket (status_id);
CREATE INDEX ticket_raised_by_idx ON ticket (raised_by);
CREATE INDEX ticket_type_id_idx   ON ticket (ticket_type_id);

CREATE UNIQUE INDEX person_email_idx ON person (lower(email));