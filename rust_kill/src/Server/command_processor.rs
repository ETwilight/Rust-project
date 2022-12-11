use crate::utils;

use tokio::net::TcpStream;

pub async fn server_send_room(client_addr: &String, room: String, id: usize) -> Result<(), ()>{
    let tcp = TcpStream::connect(client_addr).await;
    if tcp.is_err() {return Err(());}
    let writer = &mut tcp.unwrap().into_split().1;
    utils::server_write(writer, utils::encode("ROOM", room.as_str()).as_str()).await.unwrap();
    Ok(())
}