table! {
    config (c_id) {
        c_id -> Integer,
        c_name -> Text,
        c_content -> Text,
    }
}

table! {
    package (p_id) {
        p_id -> Integer,
        p_token -> Text,
        p_name -> Text,
        p_version -> Text,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        token -> Text,
        verification_code -> Nullable<Text>,
        verified -> Bool,
        is_admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    config,
    package,
    user,
);
