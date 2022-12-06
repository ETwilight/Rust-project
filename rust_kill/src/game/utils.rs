//Utils around message



use tokio::{sync::broadcast::Sender, task::JoinHandle};

use crate::Message;

fn send_mesage(queue: Sender<Message>, text:String) -> Result<JoinHandle<()>, ()>{
    let task = tokio::spawn(async move{
        let msg = Message{
            room: "lobby".to_string(),
            username: "Howdy".to_string(),
            message: text,
        };
        queue.send(msg).unwrap();
    });
    return Ok(task)
}
