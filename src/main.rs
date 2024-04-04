use rocket::{get, http::Status, launch, post, routes};

#[post("/", data = "<url>")]
fn shortten(url: &str) -> Result<String, Status> {
    short::set_url(url).map_err(|_| Status::BadRequest)
}

#[get("/<short>")]
fn get_short(short: &str) -> Option<String> {
    short::get_url(short)
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build().mount("/", routes![shortten, get_short])
}
