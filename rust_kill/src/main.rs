#[macro_use] extern crate rocket;

use rocket::{tokio::sync::broadcast::{channel, Sender, error::RecvError}, serde::{Serialize, Deserialize}, State, Shutdown, response::stream::{EventStream, Event}, fs::{relative, FileServer}};
use rocket::form::Form;
use rocket::tokio::select;


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
 struct Message{
    #[field(validate = len(..30))]
    pub room_name:String, //Maximum Length is 30 for a roomName
    #[field(validate = len(..20))]
    pub username:String, //Maximum Length is 20 for a username
    pub message:String,
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, quene: &State<Sender<Message>>){
    //A send "fails" if there are no active subscribers
    let _res = quene.send(form.into_inner());

} 

#[get("/evenets")]
async fn events(quene: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![]{
    let mut rx = quene.subscribe(); //Create a receiver
    EventStream!{
        loop{
            let msg = select! //Select wait on multiple branches and returns as soon as one of them complete
            {
                msg = rx.recv() => match msg{
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_))=> continue, //skip to the next iteration
                },
                _ = &mut end => break, //shutdown
            };
            yield Event::json(&msg);
        }
    }
}
 
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0) //Store the sender 
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static"))) //It will be saved in a folder called "static"
}

