pub struct StartGameRequest {
    num_player: i32,
    name: String,
}

pub struct StartGameResponse {
    status: String,
    reason: Option<String>,
}
