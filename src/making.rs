use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::player::PlayerInfo;

#[derive(Default)]
pub struct MatchMaking {
    player_pool: Arc<RwLock<HashMap<String, PlayerInfo>>>,
}

impl MatchMaking {
    pub async fn queue(&mut self) {
        loop {
            let players = self.match_players();
        }
    }

    pub async fn add_player(&mut self, player_info: PlayerInfo) {
        let mut pool_guard = self.player_pool.write().await;
        let player_id = player_info.player_id.to_owned();
        pool_guard.insert(player_id, player_info);
    }

    pub async fn match_players(&self) -> Vec<String> {
        let mut matched_players: Vec<String> = Vec::new();
        let pool_guard = self.player_pool.read().await;

        if pool_guard.len() % 2 == 0 {
            // No logic for now, just match whoever is there.
            // Undiscard the value when needed.
            for (k, _) in pool_guard.iter() {
                matched_players.push(k.to_string());
                if matched_players.len() == 2 {
                    break;
                }
            }
        }

        return matched_players;
    }
}
