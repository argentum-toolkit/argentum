pub fn up() -> String {
    // language=SQL
    format!(
        r#"
CREATE TABLE {prefix}authenticated
(
    id         uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    first_name varchar     NOT NULL,
    last_name  varchar     NOT NULL,
    email      varchar     NOT NULL
);

CREATE INDEX idx_authenticated_email ON {prefix}authenticated (email);

CREATE TABLE {prefix}anonymous
(
    id         uuid PRIMARY KEY,
    created_at timestamptz NOT NULL
);

CREATE TABLE {prefix}anonymous_binding
(
    user_id      uuid        NOT NULL,
    anonymous_id uuid PRIMARY KEY,
    created_at   timestamptz NOT NULL,
    CONSTRAINT fk_authenticated FOREIGN KEY (user_id) REFERENCES {prefix}authenticated (id),
    CONSTRAINT fk_anonymous FOREIGN KEY (anonymous_id) REFERENCES {prefix}anonymous (id)
);


-- CREATE SESSION DQL
CREATE TABLE {prefix}session
(
    id      uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    token   TEXT NOT NULL
);

CREATE INDEX IDX_SESSION_USER_ID ON {prefix}session (user_id);

CREATE INDEX IDX_SESSION_TOKEN ON {prefix}session (token);


"#,
        prefix = "ag_user_"
    )
}
