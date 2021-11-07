#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
mod db;
mod schema;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod hero;
use hero::{Hero};

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get("/")]
fn read() -> Json<JsonValue>  {
    println!("pring");
    Json(json!([
        "hero 1", 
        "hero 2"
    ]))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<JsonValue> {
    Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite()
    .manage(db::connect())
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}