use std::io::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub player_id: String,
    pub game_mode_id: i32,
    pub queue_region: String,
}

impl Player {
    pub async fn validate_player(&self) -> bool {
        todo!("Call auth server through RabbitMQ")
    }
}
