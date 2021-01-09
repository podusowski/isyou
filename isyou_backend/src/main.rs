#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Serialize;

#[get("/health")]
fn health() -> &'static str {
    "I'm good!"
}

#[derive(Clone, Serialize)]
struct Seek {
    id: usize,
}

#[derive(Clone, Serialize)]
struct Seeks(Vec<Seek>);

struct Global {
    seeks: std::sync::Arc<std::sync::Mutex<Seeks>>,
}

impl Default for Global {
    fn default() -> Global {
        Global {
            seeks: std::sync::Arc::new(std::sync::Mutex::new(Seeks(Vec::new()))),
        }
    }
}

#[get("/seeks")]
fn seeks(global_state: rocket::State<Global>) -> Json<Seeks> {
    Json(Seeks(global_state.seeks.lock().unwrap().0.clone()))
}

#[post("/seeks")]
fn create_seek(global_state: rocket::State<Global>) {
    global_state.seeks.lock().unwrap().0.push(Seek { id: 1 });
}

fn main() {
    rocket::ignite()
        .mount("/", routes![health, seeks, create_seek])
        .manage(Global::default())
        .launch();
}
