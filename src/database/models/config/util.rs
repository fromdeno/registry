use super::model::{Config, NewConfig};
use crate::database::schema;
use diesel::*;

pub fn set_config<'a>(
	conn: &SqliteConnection,
	key: &'a str,
	value: &'a str,
) -> Result<usize, diesel::result::Error> {
	let existing = schema::config::table.find(key);

	let existing_queried = existing.get_result::<Config>(conn);

	let new_config = NewConfig { key, value };

	match existing_queried {
		Ok(_) => {
			// the config exists in the db
			// we can update it
			diesel::update(existing).set(&new_config).execute(conn)
		}

		Err(_) => {
			// the config does not exist in the db
			// we hence need to create it
			diesel::insert_into(schema::config::table)
				.values(&new_config)
				.execute(conn)
		}
	}
}

pub fn get_config<'a>(
	conn: &SqliteConnection,
	name: &'a str,
) -> Result<String, diesel::result::Error> {
	let value = schema::config::dsl::config
		.find(name)
		.get_result::<Config>(conn)?;

	Ok(value.value)
}
