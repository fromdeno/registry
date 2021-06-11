use crate::database::schema;
use super::model::NewPackage;
use std::error::Error;
use diesel::*;


pub fn create_package<'a>(conn: &SqliteConnection, name: &'a str, version: &'a str) -> Result<usize, Box<dyn Error>> {
	let new_user = NewPackage {
		p_name: name,
		p_version: version,
		p_token: "TODO"
	};

	let result = diesel::insert_into(schema::package::table)
		.values(&new_user)
		.execute(conn)?;

	Ok(result)
}

// TODO: mechanism to store package files
