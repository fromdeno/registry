use crate::database::schema;
use super::model::NewPackage;
use std::error::Error;
use diesel::*;


pub fn create_package<'a>(conn: &SqliteConnection, name: &'a str) -> Result<usize, Box<dyn Error>> {
	let new_user = NewPackage {
		name: name,
		token: "TODO"
	};

	let result = diesel::insert_into(schema::package::table)
		.values((&new_user, schema::package::updated_at.eq(diesel::dsl::now)))
		.execute(conn)?;

	Ok(result)
}

// TODO: mechanism to store package files
