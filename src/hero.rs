use diesel;
use diesel::prelude::*;
use crate::schema::heroes;

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
}