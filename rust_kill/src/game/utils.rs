//Utils around message



use std::time::Duration;

use tokio::{sync::broadcast::Sender, task::JoinHandle, time::sleep};

use crate::Message;

use super::game_info::{Player, RoleType};

fn send_message(queue: Sender<Message>, name:String, text:String) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        let msg = Message{
            room: "rustkill".to_string(),
            username: name,
            message: text,
        };
        queue.send(msg).unwrap();
    });
    return Ok(task)
}

async fn send_dealy_mesage(queue: Sender<Message>, text:String, millisecond:u64) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        sleep(Duration::from_millis(millisecond)).await;
        let msg = Message{
            room: "rustkill".to_string(),
            username: "Howdy".to_string(),
            message: text.to_string()
        };
        queue.send(msg).unwrap();
    });
    return Ok(task)
}

fn mute(player:Player){
    
}
fn mute_all(){

}

