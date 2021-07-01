table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        content -> Varchar,
        published -> Bool,
        create_time -> Timestamp,
        last_update_time -> Timestamp,
    }
}

table! {
    posts_test (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    token_history (id) {
        id -> Int4,
        token -> Varchar,
        create_time -> Timestamp,
        expire_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    posts_test,
    token_history,
);
