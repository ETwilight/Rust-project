use tokio::{net::TcpStream, io::{self, AsyncBufReadExt}};


pub async fn connect(server_addr: &str, client_addr: &str) -> Result<TcpStream, ()>{
    let mut client = TcpStream::connect((server_addr.to_string() + ":8080").as_str()).await.unwrap();
    
    // Wrap the stream in a BufReader, so we can use the BufRead methods
    let mut reader = io::BufReader::new(&mut client);

    // Read current current data in the TcpStream
    loop{
    let received: Vec<u8> = reader.fill_buf().await.unwrap().to_vec();

    if received.len() == 0 {
        continue;
    }

    // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
    reader.consume(received.len());
    let msg = String::from_utf8(received).expect("unwrap read err");
    println!("{}", msg);

    }
    Ok(client)
}