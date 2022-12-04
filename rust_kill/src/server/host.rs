use tokio::{net::{TcpListener, TcpStream}, task::JoinHandle, io::{BufWriter, AsyncWriteExt}};

pub async fn start(server_addr: &str) -> Result<JoinHandle<()>, ()>{
    // the main thread to return
    let task = tokio::spawn(async move{
        let listener = TcpListener::bind("0.0.0.0".to_string() + ":8080").await.unwrap();
        println!("started");
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            println!("accepted");
            serverWrite(socket).await.unwrap();
            println!("written");
        }
    });
    Ok(task)
}

async fn serverWrite(socket: TcpStream) -> Result<(), ()>{
    let mut writer = BufWriter::new(socket);
    //writer.write(&(3741_i32.to_ne_bytes())).await.expect("err1");
    writer.write(b"Howdy").await.expect("err2");
    writer.flush().await.expect("err3");
    return Ok(())
}