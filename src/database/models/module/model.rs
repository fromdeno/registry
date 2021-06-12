use crate::database::models::user::model::User;
use crate::database::schema::module;
use chrono::NaiveDateTime;


#[derive(Queryable, Associations)]
#[belongs_to(User, foreign_key = "owner")]
#[table_name = "module"]
pub struct Package {
    pub id: i32,
    pub token: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub owner: String
}

#[derive(Insertable)]
#[table_name="module"]
pub struct NewPackage<'a> {
    pub token: &'a str,
    pub name: &'a str,
}