use std::sync::Arc;

use super::{
    client::Client,
    packet::{self, Packet},
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum RequestType {
    QUEUE = 0x00,
    CHECK = 0x01,
    UNQUEUE = 0x03,
    ERR = 0xFF,
}

impl TryFrom<u8> for RequestType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RequestType::QUEUE),
            0x01 => Ok(RequestType::CHECK),
            0x10 => Ok(RequestType::UNQUEUE),
            0xFF => Ok(RequestType::ERR),
            _ => Err(()),
        }
    }
}

pub struct Protocol {
    client: Arc<Client>,
}

impl Protocol {
    pub fn new(client: Arc<Client>) -> Self {
        return Self { client };
    }

    pub async fn handle_incoming(&self, packet_bytes: &[u8]) {
        if let Ok(packet) = Packet::parse_packet(&packet_bytes) {
            match packet.header.htype {
                RequestType::ERR => self.handle_error(&packet).await,
                RequestType::QUEUE => self.handle_queue(&packet).await,
                RequestType::CHECK => self.handle_check(&packet).await,
                RequestType::UNQUEUE => self.handle_unqueue(&packet).await,
            }
        }
    }

    pub async fn handle_queue(&self, packet: &Packet) {}
    pub async fn handle_check(&self, packet: &Packet) {}
    pub async fn handle_unqueue(&self, packet: &Packet) {}
    pub async fn handle_error(&self, packet: &Packet) {}
}
