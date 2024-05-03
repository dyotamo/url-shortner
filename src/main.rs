#[macro_use]
extern crate rocket;

mod web;

use web::util::{method_not_allowed, not_found, server_error};
use web::{get_short, shortten};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![shortten, get_short])
        .register("/", catchers![not_found, method_not_allowed, server_error])
}
