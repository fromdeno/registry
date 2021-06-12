use crate::database::schema::user;
use chrono::NaiveDateTime;


#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub token: String,
    pub verification_code: String,
    pub verified: bool,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub token: &'a str,
    pub verification_code: &'a str,
    pub is_verified: &'a bool,
    pub is_admin: &'a bool,
}