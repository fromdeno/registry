use crate::database::schema;
use super::model::NewModule;
use std::error::Error;
use diesel::*;


pub fn create_package<'a>(conn: &SqliteConnection, name: &'a str) -> Result<usize, Box<dyn Error>> {
	let new_user = NewModule {
		name: name,
		token: "TODO"
	};

	let result = diesel::insert_into(schema::module::table)
		.values((&new_user, schema::module::updated_at.eq(diesel::dsl::now)))
		.execute(conn)?;

	Ok(result)
}
