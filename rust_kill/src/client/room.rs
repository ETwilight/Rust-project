#[path="../utils.rs"]
mod utils;

use crate::{game_info::ClientInfo, client::game::utils::send_client_info};

use rocket::tokio::sync::broadcast::Sender;

// tell the javascript that it should connect to an empty room with name given in the room info
pub async fn connect_room(cinfo: ClientInfo, sender: Sender<ClientInfo>) {
    print!("Successfully connect to room: {} \n", cinfo.room_name);
    send_client_info(sender, cinfo.clone());
    //game::utils::send_delay_message(sender, "Howdy".to_string(), name, VisibleType::All, 15000).await.unwrap();
}