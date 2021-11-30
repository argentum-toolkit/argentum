-- ENABLE UUID EXTENSION
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


-- CREATE PASSPORT_CREDENTIALS DQL
CREATE TABLE ag_user_account_password_credential (
    user_id uuid PRIMARY KEY,
    password varchar NOT NULL,
    salt varchar NOT NULL
);


-- CREATE SESSION DQL
CREATE TABLE ag_user_account_session (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    token TEXT NOT NULL
);

CREATE INDEX IDX_SESSION_USER_ID ON ag_user_account_session(
    user_id
);

CREATE INDEX IDX_SESSION_TOKEN ON ag_user_account_session(
    token
);


-- CREATE RESTORE_PASSWORD_TOKEN DQL
CREATE TABLE ag_user_account_restore_password_token (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    token varchar NOT NULL,
    created_at timestamp with time zone NOT NULL
);

CREATE INDEX IDX_RESTORE_PASSWORD_TOKEN_USER_ID ON ag_user_account_restore_password_token(
    user_id
);

CREATE INDEX IDX_RESTORE_PASSWORD_TOKEN_TOKEN ON ag_user_account_restore_password_token(
    token
);
