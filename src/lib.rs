use std::env;

use redis::Commands as _;
use url::Url;

mod network;
mod util;

const MAX_SHORT: usize = 10;

pub fn set_url(url: &str) -> Result<String, String> {
    Url::parse(url).map_err(|err| err.to_string())?;
    let redis_server = redis_server();
    let mut con = network::connection(&redis_server);
    let short = util::generate_random_letters(MAX_SHORT);
    let _: () = con.set(&short, url).unwrap();
    Ok(short)
}

pub fn get_url(short: &str) -> Option<String> {
    let redis_server = redis_server();
    let mut con = network::connection(&redis_server);
    con.get(short).unwrap_or(None)
}

fn redis_server() -> String {
    match env::var("REDIS_SERVER") {
        Ok(var) => var,
        Err(_) => String::from("redis://localhost"),
    }
}
