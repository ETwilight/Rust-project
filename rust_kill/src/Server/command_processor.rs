use crate::server::host::client_addr;
use crate::utils;

use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;
#[path="../game.rs"]
mod game;
#[path="../game/game_info.rs"]
mod game_info;

use crate::server::command_processor::game_info::{GameState, TurnType, RevealResult};
use crate::data::{Room};
use crate::game_info::{Player, RoleType};

pub async fn server_send_room(client_addr: &String, room: String, id: usize) -> Result<(), ()>{
    let tcp = TcpStream::connect(client_addr).await;
    if tcp.is_err() {return Err(());}
    let writer = &mut tcp.unwrap().into_split().1;
    utils::server_write(writer, utils::encode("ROOM", room.as_str()).as_str()).await.unwrap();
    Ok(())
}