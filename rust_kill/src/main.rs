#[macro_use] extern crate rocket;
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message{
    #[field(validate = len(..30))]
    pub room:String, //Maximum Length is 30 for a roomName
    #[field(validate = len(..20))]
    pub username:String, //Maximum Length is 20 for a username
    pub message:String,
}

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, quene: &State<Sender<Message>>){
    //A send "fails" if there are no active subscribers
    let _res = quene.send(form.into_inner());
} 
/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    //server_addr tbd
    let server_addr = "127.0.0.1";
    let client_addr = "127.0.0.1";

    // client connection, currently in main, will be transferred
    let client = redis::Client::open("redis://".to_string()+server_addr).unwrap();
    let con = client.get_connection().unwrap();

    // a custom rocket build
    let figment = rocket::Config::figment()
        .merge(("address", client_addr))
        .merge(("port", 8000));

    let _rocket = rocket::custom(figment).mount("/", routes![/* .. */])
        .manage(channel::<Message>(1024).0) //Store the sender 
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("/static"))).launch().await?;

    Ok(())
}