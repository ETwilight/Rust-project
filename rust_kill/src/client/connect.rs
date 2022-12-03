use redis::Client;

pub async fn connect(server_addr: &str) -> Result<Client, ()>{
    let client = redis::Client::open("redis://".to_string()+server_addr).unwrap();
    let con = client.get_connection().unwrap();
    Ok(client)
}