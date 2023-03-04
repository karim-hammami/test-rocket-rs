#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[get("/")]
fn say_hello() -> &'static str {
    "Hello, welcome to the api!"
}

#[get("/heath")]
fn health_check() -> &'static str {
    "Working!"
}

pub fn get_all_todos() -> Todo {
    let id = 1;
    let title: String = String::from("gym");
    let desc: String = String::from("goto gym");
    let status = String::from("Pending");
    let new_todo = Todo {
        id,
        title,
        desc,
        status,
    };
    new_todo
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub desc: String,
    pub status: String,
}

#[get("/todos")]
pub fn get_all() -> Json<Todo> {
    Json(get_all_todos())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![say_hello, health_check, get_all])
}
