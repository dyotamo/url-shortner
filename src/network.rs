pub fn connection(server: &String) -> redis::Connection {
    let client = redis::Client::open(server.as_str()).unwrap();
    client.get_connection().unwrap()
}
