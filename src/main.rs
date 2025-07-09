// Module Declarations
mod server;
mod notes;

// Imports
use crate::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("0.0.0.0:3000");
    server.start().await;
}
