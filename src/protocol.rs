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
