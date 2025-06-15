use std::{io::Error, net::Ipv4Addr, sync::Arc};

use super::client::Client;
use crate::making::MatchMakingInstance;
use crate::network::protocol::Protocol;
use tokio::net::TcpListener;

static HOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

pub struct ServerInstance {
    stream: TcpListener,
    protocol: Arc<Protocol>,
    match_making: Arc<MatchMakingInstance>,
}

impl ServerInstance {
    pub async fn create_instance(port: u16) -> Result<Self, Error> {
        let match_making_instance = Arc::new(MatchMakingInstance::default());
        let protocol_instance = Arc::new(Protocol::new(match_making_instance.clone()));
        match TcpListener::bind((HOST, port)).await {
            Ok(tcp_stream) => {
                Ok(ServerInstance {
                    stream: tcp_stream,
                    protocol: protocol_instance,
                    match_making: match_making_instance.clone(),
                })
            }
            Err(error) => Err(error),
        }
    }

    pub async fn listen(self: Arc<Self>) {
        while let Ok((client_stream, addr)) = self.stream.accept().await {
            let client = Client::new(client_stream, addr, self.protocol.clone());
            tokio::spawn(async move {
                client.listen_incoming().await;
            });
        }
    }
}
