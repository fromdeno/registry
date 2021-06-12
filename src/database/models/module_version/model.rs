use crate::database::models::module::model::Module;
use crate::database::models::user::model::User;
use crate::database::schema::module_version;
use chrono::NaiveDateTime;

#[derive(Queryable, Associations)]
#[belongs_to(User, foreign_key = "publisher")]
#[belongs_to(Module, foreign_key = "module")]
#[table_name = "module_version"]
pub struct ModuleVersion {
	pub id: i32,
	pub version: String,
	pub created_at: NaiveDateTime,
	pub module: String,
	pub publisher: String,
}

#[derive(Insertable)]
#[table_name = "module_version"]
pub struct NewModuleVersion<'a> {
	pub version: &'a str,
	pub module: &'a str,
	pub publisher: &'a str,
}
