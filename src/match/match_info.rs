use std::collections::HashMap;
use crate::player::Player;

pub struct MatchInfo {
    pub match_id: String,
    pub game_mode_id: i32,
    pub match_region: String,
    pub players: HashMap<String, Player>
}