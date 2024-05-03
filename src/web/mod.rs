use rocket::http::Status;

pub mod util;

#[post("/", data = "<url>")]
pub fn shortten(url: &str) -> Result<String, Status> {
    short::set_url(url).map_err(|_| Status::BadRequest)
}

#[get("/<short>")]
pub fn get_short(short: &str) -> Option<String> {
    short::get_url(short)
}
