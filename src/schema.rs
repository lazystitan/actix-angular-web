table! {
    al_test (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        content -> Varchar,
        published -> Bool,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

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

allow_tables_to_appear_in_same_query!(
    al_test,
    posts,
    posts_test,
);
