use std::{io::Error, sync::Arc};

use network::server::ServerInstance;

mod making;
mod network;
mod player;
mod r#match;
mod checksum;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if let Ok(server) = ServerInstance::create_instance(8001).await {
        let server_arc = Arc::new(server);
        server_arc.listen().await;
    }

    Ok(())
}
