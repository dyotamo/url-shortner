use actix_web::{get, post, web::Bytes, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[post("/")]
async fn shortten(bytes: Bytes) -> impl Responder {
    match String::from_utf8(bytes.to_vec()) {
        Ok(url) => {
            if url.is_empty() {
                HttpResponse::BadRequest().body("The payload is empty!")
            } else {
                match short::set_url(&url.trim().to_string()) {
                    Ok(short) => HttpResponse::Ok().body(short),
                    Err(err) => HttpResponse::BadRequest().body(err),
                }
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Unable to parse the body!"),
    }
}

#[get("/{short}")]
async fn get_short(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("short").unwrap();
    if let Some(url) = short::get_url(&name.to_string()) {
        HttpResponse::Ok().body(url)
    } else {
        HttpResponse::NotFound().body(format!("{name} does not exist!"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_short).service(shortten))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
