#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/seeks")]
fn seeks() -> &'static str {
    "seeks"
}

fn main() {
    rocket::ignite().mount("/", routes![index, seeks]).launch();
}
