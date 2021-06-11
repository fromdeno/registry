use crate::database::schema::config;

#[derive(Queryable)]
pub struct Config {
    pub c_key: String,
    pub c_value: String 
}

#[derive(Insertable, AsChangeset)]
#[table_name="config"]
pub struct NewConfig<'a> {
    pub c_key: &'a str,
    pub c_value: &'a str
}