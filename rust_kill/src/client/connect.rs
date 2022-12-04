use tokio::{net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}, io::{AsyncBufReadExt, BufReader}};
#[path="../utils.rs"]
mod utils;


pub async fn connect(server_addr: &str, client_addr: &str) -> Result<(OwnedReadHalf, OwnedWriteHalf), ()>{
    let mut clt = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    let mut client = clt.into_split();
    utils::clientWrite(&mut client.1, "client_addr".to_string()).await.unwrap();
    println!("a");
    // Wrap the stream in a BufReader, so we can use the BufRead methods
    let mut reader = BufReader::new(&mut client.0);
    // Read current current data in the TcpStream
    loop{
        let raw= reader.fill_buf().await;
        if raw.is_ok() {
            let received = raw.unwrap().to_vec();
            println!("client get size: {}", received.len());
            if received.len() == 0 {
                continue;
            }
            // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
            reader.consume(received.len());
            let msg = String::from_utf8(received).expect("unwrap read err");
            println!("client get: {}", msg);
        }
    }
    Ok(client)
}