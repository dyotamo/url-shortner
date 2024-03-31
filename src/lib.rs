use std::env;

use redis::Commands as _;
use url::Url;

mod network;
mod util;

const MAX_SHORT: usize = 10;

pub fn set_url(url: &String) -> Result<String, String> {
    match Url::parse(url.as_str()) {
        Ok(_) => {
            let redis_server = env::var("REDIS_SERVER").unwrap();
            let mut con = network::connection(&redis_server);
            let short = util::generate_random_letters(MAX_SHORT);
            let _: () = con.set(&short, url).unwrap();
            Ok(short)
        }
        Err(_) => Err(format!("[{}] is not a valid URL!", url)),
    }
}

pub fn get_url(short: &String) -> Option<String> {
    let redis_server = env::var("REDIS_SERVER").unwrap();
    let mut con = network::connection(&redis_server);
    con.get(short).unwrap_or(None)
}
