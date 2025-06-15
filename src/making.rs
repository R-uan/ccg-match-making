use crate::utils::logger::Logger;
use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{sync::RwLock, time};
use crate::logger;
use crate::r#match::match_info::MatchInfo;
use crate::player::Player;

#[derive(Default)]
pub struct MatchMakingInstance {
    matches: Arc<RwLock<HashMap<String, MatchInfo>>>,
    player_pool: Arc<RwLock<HashMap<String, Arc<Player>>>>,
}

impl MatchMakingInstance {
    pub async fn queue_player(&self, player: Player) -> String {
        logger!(INFO, "`{}` has been queued up.", &player.player_id);
        let mut pool = self.player_pool.write().await;
        pool.insert(player.player_id.clone(), Arc::new(player));
        todo!("Generate token string for cancellation")
    }

    pub async fn queue(&mut self) {
        loop {
            if let Some(players) = self.match_players().await {
                let mut pool_guard = self.player_pool.write().await;
            }
            time::interval(Duration::from_secs(1)).tick().await;
        }
    }

    pub async fn match_players(&self) -> Option<Vec<Arc<Player>>> {
        let mut matched_players: Vec<Arc<Player>> = Vec::new();
        let mut pool_guard = self.player_pool.write().await;

        if pool_guard.len() >= 2 {
            for (_, player) in pool_guard.iter() {
                if matched_players.len() == 2 {
                    break;
                } else {
                    matched_players.push(player.clone());
                }
            }

            for player in matched_players.iter() {
                pool_guard.remove(player.player_id.as_str());
            }
            
            return Some(matched_players);
        }
        None
    }
}
