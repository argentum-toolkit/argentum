//table_name_prefix = "ag_user_account_"
pub fn up(table_name_prefix: &str) -> Vec<String> {
    let mut queries = vec![];
    // language=SQL

    // ENABLE UUID EXTENSION
    queries.push("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";".to_string());

    //CREATE OIDC_CREDENTIALS DQL
    queries.push(format!(
        r#"
            CREATE TABLE {prefix}credential (
                external_id varchar NOT NULL,
                user_id uuid NOT NULL,
                provider varchar NOT NULL,
                created_at timestamp with time zone NOT NULL
            );
        "#,
        prefix = table_name_prefix
    ));

    queries.push(format!(
        "CREATE UNIQUE INDEX IDX_{idx_prefix}CREDENTIAL ON {prefix}credential (external_id, user_id, provider);",
        prefix = table_name_prefix,
        idx_prefix = table_name_prefix.to_uppercase(),
    ));

    queries
}
