use std::{sync::Arc, io::{Read, Write, self}};

use tokio::{net::{TcpStream, tcp::OwnedWriteHalf}, io::{BufWriter, AsyncWriteExt, AsyncReadExt, AsyncRead, AsyncWrite}};

pub async fn serverWrite(socket: &mut OwnedWriteHalf, msg: String) -> Result<(), ()>{
    let mut writer = BufWriter::new(socket);
    //writer.write(&(3741_i32.to_ne_bytes())).await.expect("err1");
    writer.write(msg.as_bytes()).await.expect("err2");
    writer.flush().await.expect("err3");
    println!("send from server");
    return Ok(())
}

pub async fn clientWrite(socket: &mut OwnedWriteHalf, msg: String) -> Result<(), ()>{
    let mut writer = BufWriter::new(socket);
    //writer.write(&(3741_i32.to_ne_bytes())).await.expect("err1");
    writer.write(msg.as_bytes()).await.expect("err2");
    writer.flush().await.expect("err3");
    println!("send from client");
    return Ok(())
}
