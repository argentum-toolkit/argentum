//table_name_prefix = "ag_user_account_"
pub fn up(table_name_prefix: &str) -> Vec<String> {
    let mut queries = vec![];
    // language=SQL

    // ENABLE UUID EXTENSION
    queries.push("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";".to_string());

    //CREATE PASSPORT_CREDENTIALS DQL
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}password_credential (
                user_id  uuid PRIMARY KEY,
                password varchar NOT NULL,
                salt     varchar NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    // CREATE RESTORE_PASSWORD_TOKEN DQL
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}restore_password_token (
                id uuid PRIMARY KEY,
                user_id uuid NOT NULL,
                token varchar NOT NULL,
                created_at timestamp with time zone NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE INDEX IDX_RESTORE_PASSWORD_TOKEN_USER_ID ON {prefix}restore_password_token (user_id);",
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE INDEX IDX_RESTORE_PASSWORD_TOKEN_TOKEN ON {prefix}restore_password_token (token);",
        prefix = table_name_prefix
    ));

    queries
}
