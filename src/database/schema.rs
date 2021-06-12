table! {
	config (key) {
		key -> Text,
		value -> Text,
	}
}

table! {
	module (id) {
		id -> Text,
		token -> Text,
		name -> Text,
		created_at -> Timestamp,
		updated_at -> Timestamp,
		owner -> Text,
	}
}

table! {
	module_version (id) {
		id -> Nullable<Text>,
		version -> Text,
		created_at -> Timestamp,
		module -> Text,
		publisher -> Text,
	}
}

table! {
	user (id) {
		id -> Text,
		username -> Text,
		email -> Text,
		password -> Text,
		token -> Text,
		verification_code -> Nullable<Text>,
		is_verified -> Bool,
		is_admin -> Bool,
		created_at -> Timestamp,
		updated_at -> Timestamp,
	}
}

joinable!(module -> user (owner));
joinable!(module_version -> module (module));
joinable!(module_version -> user (publisher));

allow_tables_to_appear_in_same_query!(config, module, module_version, user,);
