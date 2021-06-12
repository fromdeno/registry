use crate::database::schema::config;

#[derive(Queryable)]
pub struct Config {
	pub key: String,
	pub value: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "config"]
pub struct NewConfig<'a> {
	pub key: &'a str,
	pub value: &'a str,
}
