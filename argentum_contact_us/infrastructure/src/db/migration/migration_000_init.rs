//table_name_prefix = "ag_contact_us_"
pub fn up(table_name_prefix: &str) -> Vec<String> {
    let mut queries = vec![];
    // language=SQL

    // ENABLE UUID EXTENSION
    queries.push("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";".to_string());

    // CREATE MESSAGE DQL
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}message (
                id uuid PRIMARY KEY,
                subject varchar(255) NOT NULL,
                message text NOT NULL,
                user_id uuid NULL,
                name varchar NULL,
                email varchar NULL,
                created_at timestamp with time zone NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE INDEX IDX_MESSAGE_USER_ID ON {prefix}message (user_id);",
        prefix = table_name_prefix
    ));

    queries
}
