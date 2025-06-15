use std::{net::SocketAddr, sync::Arc};

use super::protocol::Protocol;
use tokio::io::AsyncWriteExt;
use tokio::{
    io::AsyncReadExt,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{Mutex, RwLock},
};

pub struct Client {
    addr: SocketAddr,
    protocol: Arc<Protocol>,
    connected: Arc<RwLock<bool>>,
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr, protocol: Arc<Protocol>) -> Arc<Self> {
        let (r, w) = stream.into_split();
        Arc::new(Self {
            addr,
            protocol,
            read_stream: Arc::new(Mutex::new(r)),
            write_stream: Arc::new(Mutex::new(w)),
            connected: Arc::new(RwLock::new(true)),
        })
    }

    pub async fn listen_incoming(self: Arc<Self>) {
        let mut buffer = [0; 1024];
        while *self.connected.read().await {
            let mut read_stream_lock = self.read_stream.lock().await;
            let bytes_read = match read_stream_lock.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => n,
                _ => break,
            };

            self.protocol
                .handle_incoming(self.clone(), &buffer[..bytes_read])
                .await;
        }
    }

    pub async fn send_packet(&self, packet: Box<[u8]>) {
        self.write_stream
            .lock()
            .await
            .write_all(&packet)
            .await
            .unwrap();
    }
}
