use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::AsyncReadExt,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{Mutex, RwLock},
};

use super::protocol::Protocol;

pub struct Client {
    addr: SocketAddr,
    connected: Arc<RwLock<bool>>,
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Arc<Self> {
        let (r, w) = stream.into_split();
        return Arc::new(Self {
            addr,
            read_stream: Arc::new(Mutex::new(r)),
            write_stream: Arc::new(Mutex::new(w)),
            connected: Arc::new(RwLock::new(true)),
        });
    }

    pub async fn listen_incoming(self: Arc<Self>) {
        let mut buffer = [0; 1024];

        let client_clone = Arc::clone(&self);
        let protocol = Protocol::new(client_clone);
        while *self.connected.read().await {
            let mut read_stream_lock = self.read_stream.lock().await;
            let bytes_read = match read_stream_lock.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => n,
                _ => break,
            };

            protocol.handle_incoming(&buffer[..bytes_read]).await;
        }
    }
}
