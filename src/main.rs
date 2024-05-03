use rocket::{http::Status, response::status, Request};

#[macro_use]
extern crate rocket;

#[post("/", data = "<url>")]
pub fn shortten(url: &str) -> Result<String, Status> {
    short::set_url(url).map_err(|_| Status::BadRequest)
}

#[get("/<short>")]
pub fn get_short(short: &str) -> Option<String> {
    short::get_url(short)
}

#[catch(404)]
pub fn not_found<'a>(_: &'a Request) -> status::NotFound<&'a str> {
    status::NotFound("oops, not found")
}

#[catch(405)]
pub fn method_not_allowed<'a>(_: &'a Request) -> status::Custom<&'a str> {
    status::Custom(Status::MethodNotAllowed, "oops, method not allowed")
}

#[catch(500)]
pub fn server_error<'a>(_: &'a Request) -> status::Custom<&'a str> {
    status::Custom(Status::InternalServerError, "oops, internal server error")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![shortten, get_short])
        .register("/", catchers![not_found, method_not_allowed, server_error])
}
