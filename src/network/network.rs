use std::{io::Error, net::Ipv4Addr, sync::Arc};

use tokio::net::TcpListener;

use super::client::Client;

static HOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

pub struct ServerInstance {
    stream: TcpListener,
}

impl ServerInstance {
    pub async fn create_instance(port: u16) -> Result<Self, Error> {
        return match TcpListener::bind((HOST, port)).await {
            Ok(tcp_stream) => {
                return Ok(ServerInstance { stream: tcp_stream });
            }
            Err(error) => Err(error),
        };
    }

    pub async fn listen(self: Arc<Self>) {
        while let Ok((client_stream, addr)) = self.stream.accept().await {
            let client = Client::new(client_stream, addr);
            tokio::spawn(async move {
                client.listen_incoming().await;
            });
        }
    }
}
