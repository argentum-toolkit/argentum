table! {
    ag_user_anonymous (id) {
        id -> Uuid,
        created_at -> Timestamptz,
    }
}

table! {
    ag_user_anonymous_binding (anonymous_id) {
        user_id -> Uuid,
        anonymous_id -> Uuid,
        created_at -> Timestamptz,
    }
}

table! {
    ag_user_authenticated (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
    }
}

joinable!(ag_user_anonymous_binding -> ag_user_anonymous (anonymous_id));
joinable!(ag_user_anonymous_binding -> ag_user_authenticated (user_id));

allow_tables_to_appear_in_same_query!(
    ag_user_anonymous,
    ag_user_anonymous_binding,
    ag_user_authenticated,
);
