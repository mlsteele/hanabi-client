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
pub struct StartGameResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinGameRequest {
    pub game_name: String,
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinGameResponse {
    pub session: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStateRequest {
    pub session: String,
    pub wait: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStateResponse {
    pub state: GameStateSummary,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GameState {
    NotStarted,
    WaitingForTurn,
    YourTurn,
    Finished,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateSummary {
    pub state: GameState,
    pub players: Vec<String>,
    pub hand: Vec<HiddenCard>,
    pub other_hands: HashMap<String, Vec<Card>>,
    pub board: HashMap<Color, Vec<Card>>,
    pub discard: Vec<Card>,
    pub turns: Vec<i32>, // TODO
    pub turn_cursor: i32,
}

type Color = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: i32,
    pub color: Color,
    pub number: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HiddenCard {
    pub id: i32,
}
