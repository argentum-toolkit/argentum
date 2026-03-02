//table_name_prefix = "ag_user_"
pub fn up(table_name_prefix: &str) -> Vec<String> {
    let mut queries = vec![];
    // language=SQL

    // ENABLE UUID EXTENSION
    queries.push("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";".to_string());

    // MEMBER USER
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}member (
                id uuid PRIMARY KEY,
                created_at timestamptz NOT NULL,
                first_name varchar NOT NULL,
                last_name varchar,
                additional_name varchar,
                email varchar NOT NULL UNIQUE
            );
        "#,
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE INDEX idx_member_email ON {prefix}member (email);",
        prefix = table_name_prefix
    ));

    // GUEST USER
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}guest (
                id uuid PRIMARY KEY,
                created_at timestamptz NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    // BINDING GUESTS TO MEMBERS
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}guest_binding (
                member_id uuid NOT NULL,
                guest_id uuid PRIMARY KEY,
                created_at timestamptz NOT NULL,
                CONSTRAINT fk_member FOREIGN KEY (member_id) REFERENCES {prefix}member (id),
                CONSTRAINT fk_guest FOREIGN KEY (guest_id) REFERENCES {prefix}guest (id)
            );
        "#,
        prefix = table_name_prefix
    ));

    // SESSION
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}session
            (
                id uuid PRIMARY KEY,
                user_id uuid NOT NULL,
                created_at timestamptz NOT NULL,
                token TEXT NOT NULL
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
