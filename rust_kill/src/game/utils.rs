//Utils around message



use std::time::Duration;

use rocket::serde::json::Json;
use tokio::{sync::broadcast::Sender, task::JoinHandle, time::sleep};

use crate::{Message, VisibleType};

use super::game_info::{Player, Room};

pub fn send_message(queue: Sender<Json<Message>>, name:String, text:String, visible_type:VisibleType) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        let msg = Message{
            room: "rustkill".to_string(),
            username: name,
            message: text,
            visible_type
        };
        queue.send(Json(msg)).unwrap();
    });
    return Ok(task)
}

pub async fn send_delay_message(queue: Sender<Json<Message>>, name:String, text:String, visible_type:VisibleType, millisecond:u64) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        sleep(Duration::from_millis(millisecond)).await;
        let msg = Message{
            room: "rustkill".to_string(),
            username: name.clone(),
            message: text.to_string(),
            visible_type
        };
        queue.send(Json(msg)).unwrap();
        sleep(Duration::from_millis(1000)).await;
    });
    return Ok(task)
}

pub fn send_room(queue: Sender<Room>, room: Room) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        queue.send(room).unwrap();
    });
    return Ok(task)
}

pub fn mute(player:Player){
    
}
pub fn mute_all(){

}

