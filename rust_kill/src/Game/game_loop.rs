use rocket::serde::{Serialize, Deserialize};


use super::game_info::Room;



pub fn update(room:&mut Room){
    match room.game_state.turn.turn_state{
        super::game_info::TurnType::StartTurn => {
            assign_role();
        },
        super::game_info::TurnType::WolfTurn => todo!(),
        super::game_info::TurnType::WitchTurn => todo!(),
        super::game_info::TurnType::ProphetTurn => todo!(),
        super::game_info::TurnType::SpeakTurn => todo!(),
        super::game_info::TurnType::VoteTurn => todo!(),
        super::game_info::TurnType::LastWordTurn => todo!(),
        super::game_info::TurnType::EndTurn => todo!(),
    }
    room.game_state.turn.turn_state = room.game_state.turn.turn_state.next();
}

pub fn assign_role(){

}