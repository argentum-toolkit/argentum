// @generated automatically by Diesel CLI.

diesel::table! {
    ag_user_account_password_credential (user_id) {
        user_id -> Uuid,
        password -> Varchar,
        salt -> Varchar,
    }
}

diesel::table! {
    ag_user_account_restore_password_token (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ag_user_account_password_credential,
    ag_user_account_restore_password_token,
);
