#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Serialize;

#[get("/health")]
fn health() -> &'static str {
    "I'm good!"
}

#[derive(Clone, Serialize, serde::Deserialize)]
struct Point {
    lat: f64,
    lon: f64,
}

#[derive(Clone, Serialize)]
struct Seek {
    id: usize,
    points: Vec<Point>,
}

impl Seek {
    fn new() -> Seek {
        Seek {
            id: 1,
            points: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize)]
struct Seeks(Vec<Seek>);

struct Global {
    seeks_by_id: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<usize, Seek>>>,
}

impl Default for Global {
    fn default() -> Global {
        Global {
            seeks_by_id: std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
        }
    }
}

#[derive(Clone, Serialize)]
struct SeekIds(Vec<usize>);

#[get("/seeks")]
fn seeks(global_state: rocket::State<Global>) -> Json<SeekIds> {
    let ids = global_state
        .seeks_by_id
        .lock()
        .unwrap()
        .keys()
        .map(|&x| x)
        .collect();
    Json(SeekIds(ids))
}

#[post("/seeks")]
fn create_seek(global_state: rocket::State<Global>) {
    let mut seeks_by_id = global_state.seeks_by_id.lock().unwrap();
    seeks_by_id.insert(1, Seek::new());
}

#[post("/seeks/<id>/points")]
fn create_seek_point(global_state: rocket::State<Global>, id: usize) {
    let mut seeks_by_id = global_state.seeks_by_id.lock().unwrap();
    seeks_by_id
        .entry(id)
        .and_modify(|seek| seek.points.push(Point { lat: 1.0, lon: 1.0 }));
}

#[derive(Clone, Serialize)]
struct SeekPoints(Vec<Point>);

#[get("/seeks/<id>/points")]
fn get_seek_points(global_state: rocket::State<Global>, id: usize) -> Json<SeekPoints> {
    let seeks_by_id = global_state.seeks_by_id.lock().unwrap();
    Json(SeekPoints(seeks_by_id[&id].points.clone()))
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                health,
                seeks,
                create_seek,
                create_seek_point,
                get_seek_points
            ],
        )
        .manage(Global::default())
        .launch();
}
