use tokio::{net::TcpStream, task::JoinHandle};
#[path="../utils.rs"]
mod utils;


pub async fn connect(server_addr: &str, client_addr: &str) -> Result<JoinHandle<()>, ()>{
    let mut clt = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    let (mut reader, mut writer) = clt.into_split();
    utils::clientWrite(&mut writer, client_addr).await.unwrap();
    // Wrap the stream in a BufReader, so we can use the BufRead methods
    //let mut reader = BufReader::new(&mut client.0);
    // Read current current data in the TcpStream
    let client = tokio::spawn(async move{
        utils::clientRead(&mut reader, &mut writer, "client get: ").await;
    });
    Ok(client)
}