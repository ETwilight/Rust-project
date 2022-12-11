//Utils around message



use std::time::Duration;

use rocket::serde::json::Json;
use tokio::{sync::broadcast::Sender, task::JoinHandle, time::sleep};



use crate::data::{Message, VisibleType, Room};

use super::game_info::{Player};

pub fn send_message(queue: Sender<Message>, name:String, text:String, visible_type:VisibleType) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        let msg = Message{
            username: name,
            message: text,
            visible_type
        };
        queue.send(msg).unwrap();
    });
    return Ok(task)
}

pub async fn send_delay_message(queue: Sender<Message>, name:String, text:String, visible_type:VisibleType, millisecond:u64) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        sleep(Duration::from_millis(millisecond)).await;
        let msg = Message{
            username: name.clone(),
            message: text.to_string(),
            visible_type
        };
        queue.send(msg).unwrap();
    });
    return Ok(task)
}

pub fn send_room(queue: Sender<Room>, room: Room) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        queue.send(room).unwrap();
    });
    return Ok(task)
}

pub async fn send_delay_room(queue: Sender<Room>, room: Room, millisecond:u64) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        sleep(Duration::from_millis(millisecond)).await;
        queue.send(room).unwrap();
    });
    return Ok(task)
}


pub fn mute(player:Player){
    
}
pub fn mute_all(){

}

