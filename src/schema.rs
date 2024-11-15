// @generated automatically by Diesel CLI.

diesel::table! {
    message (id) {
        id -> Varchar,
        from -> Varchar,
        to -> Varchar,
        action -> Varchar,
        status -> Varchar,
        info -> Json,
        created_at -> Timestamp,
    }
}

diesel::table! {
    record (id) {
        id -> Varchar,
        event_id -> Varchar,
        relay -> Varchar,
        message_id -> Varchar,
        status -> Varchar,
        info -> Json,
        created_at -> Timestamp,
    }
}

diesel::table! {
    relays (id) {
        id -> Varchar,
        url -> Varchar,
        info -> Json,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    message,
    record,
    relays,
);
