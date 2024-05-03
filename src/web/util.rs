use rocket::{http::Status, response::status, Request};

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
