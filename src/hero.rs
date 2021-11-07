use diesel;
use diesel::prelude::*;
use crate::schema::heroes;

use diesel::pg::PgConnection;


#[table_name = "heroes"]
#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
}


impl Hero {
    pub fn create(hero: Hero, connection: &PgConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<Hero> {
        heroes::table.order(heroes::id.asc()).load::<Hero>(connection).unwrap()
    }

    pub fn update(id: i32, hero: Hero, connection: &PgConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
    }
}