
CREATE TABLE ag_user_authenticated (
    id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    first_name varchar NOT NULL,
    last_name varchar NOT NULL,
    email varchar NOT NULL
);

CREATE INDEX idx_authenticated_email ON ag_user_authenticated(
    email
);

CREATE TABLE ag_user_anonymous (
     id uuid PRIMARY KEY,
     created_at timestamptz NOT NULL
);

CREATE TABLE ag_user_anonymous_binding (
    user_id uuid NOT NULL,
    anonymous_id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    CONSTRAINT fk_authenticated
        FOREIGN KEY(user_id)
            REFERENCES ag_user_authenticated(id),
    CONSTRAINT fk_anonymous
        FOREIGN KEY(anonymous_id)
            REFERENCES ag_user_anonymous(id)
);
