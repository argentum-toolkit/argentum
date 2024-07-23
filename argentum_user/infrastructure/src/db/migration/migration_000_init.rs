//table_name_prefix = "ag_user_"
pub fn up(table_name_prefix: &str) -> Vec<String> {
    let mut queries = vec![];
    // language=SQL

    // ENABLE UUID EXTENSION
    queries.push("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";".to_string());

    // AUTHENTICATED USER
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}authenticated (
                id         uuid PRIMARY KEY,
                created_at timestamptz NOT NULL,
                first_name varchar     NOT NULL,
                last_name  varchar     NOT NULL,
                email      varchar     NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE INDEX idx_authenticated_email ON {prefix}authenticated (email);",
        prefix = table_name_prefix
    ));

    // ANONYMOUS USER
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}anonymous (
                id         uuid PRIMARY KEY,
                created_at timestamptz NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    // BINDING ANONYMOUS TO AUTHENTICATED USER
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}anonymous_binding (
                user_id      uuid        NOT NULL,
                anonymous_id uuid PRIMARY KEY,
                created_at   timestamptz NOT NULL,
                CONSTRAINT fk_authenticated FOREIGN KEY (user_id) REFERENCES {prefix}authenticated (id),
                CONSTRAINT fk_anonymous FOREIGN KEY (anonymous_id) REFERENCES {prefix}anonymous (id)
            );
        "#,
        prefix = table_name_prefix
    ));

    // SESSION
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}session
            (
                id      uuid PRIMARY KEY,
                user_id uuid NOT NULL,
                token   TEXT NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE INDEX IDX_SESSION_USER_ID ON {prefix}session (user_id);",
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE INDEX IDX_SESSION_TOKEN ON {prefix}session (token);",
        prefix = table_name_prefix
    ));

    queries
}
