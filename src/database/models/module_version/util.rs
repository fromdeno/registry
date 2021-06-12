use super::model::NewModuleVersion;
use crate::database::schema;
use diesel::*;
use std::error::Error;

pub fn create_module_version<'a>(
	conn: &SqliteConnection,
	version: &'a str,
	module: &'a str,
	publisher: &'a str,
) -> Result<usize, Box<dyn Error>> {
	let new_module_version = NewModuleVersion {
		version,
		module,
		publisher,
	};

	let result = diesel::insert_into(schema::module_version::table)
		.values(&new_module_version)
		.execute(conn)?;

	Ok(result)
}
