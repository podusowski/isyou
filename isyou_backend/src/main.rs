#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/health")]
fn health() -> &'static str {
    "I'm good!"
}

#[post("/seeks")]
fn seeks() -> &'static str {
    "seeks"
}

fn main() {
    rocket::ignite().mount("/", routes![health, seeks]).launch();
}
