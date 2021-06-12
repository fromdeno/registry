use super::model::NewUser;
use crate::database::schema;
use diesel::*;
use std::error::Error;

pub fn create_user<'a>(
	conn: &SqliteConnection,
	username: &'a str,
	email: &'a str,
	password: &'a str,
) -> Result<usize, Box<dyn Error>> {
	let new_user = NewUser {
		username,
		email,
		password,
		token: "TODO",
		verification_code: "TODO",
		is_verified: &false,
		is_admin: &false,
	};

	let result = diesel::insert_into(schema::user::table)
		.values((&new_user, schema::user::updated_at.eq(dsl::now)))
		.execute(conn)?;

	Ok(result)
}
