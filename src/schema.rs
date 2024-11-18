// @generated automatically by Diesel CLI.

diesel::table! {
    clock (id) {
        id -> Varchar,
        value -> Int8,
        info -> Json,
        created_at -> Timestamp,
    }
}

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
    message_clock (id) {
        id -> Varchar,
        from -> Varchar,
        from_clock -> Int8,
        to -> Varchar,
        to_clock -> Int8,
        action -> Varchar,
        status -> Varchar,
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
    clock,
    message,
    message_clock,
    record,
    relays,
);
