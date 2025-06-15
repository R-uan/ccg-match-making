use super::{
    client::Client,
    packet::{self, Packet},
};
use crate::logger;
use crate::making::MatchMakingInstance;
use crate::player::Player;
use crate::utils::logger::Logger;
use std::sync::Arc;
use crate::r#match::responses::{ErrResponse, QueueResponse};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum HeaderType {
    QUEUE = 0x00,
    CHECK = 0x01,
    CANCEL = 0x02,
    TOKEN = 0x03,
    
    AUTH_ERR = 0xF1,
    PACKET_ERR = 0xF2,
    PLAYER_ERR = 0xF3,
    MATCH_ERR = 0xF4,
    ERR = 0xFF,
}

impl TryFrom<u8> for HeaderType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(HeaderType::QUEUE),
            0x01 => Ok(HeaderType::CHECK),
            0x02 => Ok(HeaderType::CANCEL),
            0x03 => Ok(HeaderType::TOKEN),
            0xF1 => Ok(HeaderType::AUTH_ERR),
            0xF2 => Ok(HeaderType::PACKET_ERR),
            0xF3 => Ok(HeaderType::PLAYER_ERR),
            0xF4 => Ok(HeaderType::MATCH_ERR),
            0xFF => Ok(HeaderType::ERR),
            _ => Err(()),
        }
    }
}

pub struct Protocol {
    match_making_instance: Arc<MatchMakingInstance>,
}

impl Protocol {
    pub fn new(match_making_instance: Arc<MatchMakingInstance>) -> Self {
        Self {
            match_making_instance,
        }
    }

    pub async fn handle_incoming(&self, client: Arc<Client>, packet_bytes: &[u8]) {
        if let Ok(packet) = Packet::parse_packet(&packet_bytes) {
            match packet.header.header_type {
                HeaderType::ERR => self.handle_error(&packet).await,
                HeaderType::QUEUE => self.handle_queue(client, &packet).await,
                HeaderType::CHECK => self.handle_check(&packet).await,
                HeaderType::CANCEL => self.handle_unqueue(&packet).await,
                _ => {}
            }
        }
    }

    pub async fn handle_queue(&self, client: Arc<Client>, packet: &Packet) {
        match serde_cbor::from_slice::<Player>(&packet.payload) {
            Ok(player) => {
                if player.validate_player().await {
                    let token = self.match_making_instance.queue_player(player).await;
                    let payload = QueueResponse::new(true, "Player queued", &token);
                    let packet = Packet::new(HeaderType::TOKEN, serde_cbor::to_vec(&payload).unwrap());
                    client.send_packet(packet.wrap_packet()).await;
                    return;
                } else {
                    let payload = ErrResponse::new(3, "Failed to validate player");
                    let packet = Packet::new(HeaderType::PLAYER_ERR, serde_cbor::to_vec(&payload).unwrap());
                    client.send_packet(packet.wrap_packet()).await;   
                    return;
                }
            }
            Err(error) => {
                let error_string = error.to_string();
                logger!(ERROR, "Failed to parse player: {error_string}");
                let payload = ErrResponse::new(3, &error_string);
                let packet = Packet::new(HeaderType::PLAYER_ERR, serde_cbor::to_vec(&payload).unwrap());
                client.send_packet(packet.wrap_packet()).await;
                return;
            }
        }
    }
    pub async fn handle_check(&self, packet: &Packet) {}
    pub async fn handle_unqueue(&self, packet: &Packet) {}
    pub async fn handle_error(&self, packet: &Packet) {}
}
