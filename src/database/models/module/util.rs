use super::model::NewModule;
use crate::database::schema;
use diesel::*;
use std::error::Error;

pub fn create_package<'a>(
	conn: &SqliteConnection,
	name: &'a str,
	owner: &'a str,
) -> Result<usize, Box<dyn Error>> {
	let new_user = NewModule {
		name,
		token: "TODO",
		owner,
	};

	let result = diesel::insert_into(schema::module::table)
		.values((&new_user, schema::module::updated_at.eq(diesel::dsl::now)))
		.execute(conn)?;

	Ok(result)
}
