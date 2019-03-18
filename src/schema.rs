table! {
    users (id) {
        id -> Int4,
        username -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    visits (id) {
        id -> Int4,
        user_id -> Int4,
        enter_at -> Date,
        exit_at -> Date,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(visits -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    visits,
);
