use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericResponse {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartGameRequest {
    pub num_players: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartGameResponse {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinGameRequest {
    pub game_name: String,
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinGameResponse {
    pub status: String,
    pub reason: Option<String>,
    pub session: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStateRequest {
    session: String,
    wait: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStateResponse {
    status: String,
    reason: String,
    state: GameStateSummary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateSummary {
    state: String,
    players: Vec<String>,
    hand: Vec<HiddenCard>,
    other_hands: HashMap<String, Vec<Card>>,
    board: HashMap<Color, Vec<Card>>,
    discard: Vec<Card>,
    turns: Vec<i32>, // TODO
    turn_cursor: i32,
}

type Color = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    id: i32,
    color: Color,
    number: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HiddenCard {
    id: i32,
}
