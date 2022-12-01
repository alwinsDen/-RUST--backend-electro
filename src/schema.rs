// @generated automatically by Diesel CLI.

diesel::table! {
    activity_tracker (id) {
        id -> Int8,
        user_id -> Uuid,
        related_info -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    org_projects (id) {
        id -> Uuid,
        display_name -> Varchar,
        setting -> Json,
        related_files -> Array<Nullable<Text>>,
        user_id -> Uuid,
        created_at -> Timestamp,
        path -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        created_at -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    activity_tracker,
    org_projects,
    users,
);
