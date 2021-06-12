use crate::database::models::user::model::User;
use crate::database::schema::package;
use chrono::NaiveDateTime;


#[derive(Queryable, Associations)]
#[belongs_to(User, foreign_key = "owner")]
#[table_name = "package"]
pub struct Package {
    pub id: i32,
    pub token: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub owner: String
}

#[derive(Insertable)]
#[table_name="package"]
pub struct NewPackage<'a> {
    pub token: &'a str,
    pub name: &'a str,
}