#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;

use serde::Serialize;

#[get("/health")]
fn health() -> &'static str {
    "I'm good!"
}

#[derive(Serialize)]
struct Seek {
    id: usize,
}

#[derive(Serialize)]
struct Seeks(Vec<Seek>);

#[get("/seeks")]
fn seeks() -> Json<Seeks> {
    Json(Seeks(vec![]))
}

#[post("/seeks")]
fn create_seek() {}

fn main() {
    rocket::ignite()
        .mount("/", routes![health, seeks, create_seek])
        .launch();
}
