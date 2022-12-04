use tokio::{net::TcpStream, io::{BufWriter, AsyncWriteExt}};

pub async fn serverWrite(socket: TcpStream) -> Result<(), ()>{
    let mut writer = BufWriter::new(socket);
    //writer.write(&(3741_i32.to_ne_bytes())).await.expect("err1");
    writer.write(b"Howdy").await.expect("err2");
    writer.flush().await.expect("err3");
    return Ok(())
}