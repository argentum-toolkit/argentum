pub fn up() -> String {
    // language=SQL
    format!(
        r#"
-- ENABLE UUID EXTENSION
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- CREATE PASSPORT_CREDENTIALS DQL
CREATE TABLE {prefix}password_credential
(
    user_id  uuid PRIMARY KEY,
    password varchar NOT NULL,
    salt     varchar NOT NULL
);

-- CREATE RESTORE_PASSWORD_TOKEN DQL
CREATE TABLE {prefix}restore_password_token
(
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    token varchar NOT NULL,
    created_at timestamp with time zone NOT NULL
);

CREATE INDEX IDX_RESTORE_PASSWORD_TOKEN_USER_ID ON  {prefix}restore_password_token (user_id);

CREATE INDEX IDX_RESTORE_PASSWORD_TOKEN_TOKEN ON  {prefix}restore_password_token (token);

"#,
        prefix = "ag_user_account_"
    )
}
