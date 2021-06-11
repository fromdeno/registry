use crate::database::schema::package;


#[derive(Queryable)]
pub struct Package {
    pub p_id: i32,
    pub p_token: String,
    pub p_name: String,
    pub p_version: String,
}

#[derive(Insertable)]
#[table_name="package"]
pub struct NewPackage<'a> {
    pub p_token: &'a str,
    pub p_name: &'a str,
    pub p_version: &'a str
}