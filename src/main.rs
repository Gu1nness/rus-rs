#[macro_use]
extern crate rocket;
extern crate rand;

mod service;
mod models;
mod schema;
mod catchers;

use service::{create, get, stage};
use catchers::not_found;

#[launch]
fn rocket() -> _{
    rocket::build()
    .mount("/shorten", routes![create])
    .mount("/", routes![get])
    .register("/", catchers![not_found])
    .attach(stage())
}
