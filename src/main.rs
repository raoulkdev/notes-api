// Module Declarations
mod notes;
mod server;

// Imports
use crate::server::Server;

#[tokio::main]
async fn main() {
    // Create a new server
    let server = Server::new("0.0.0.0:3000");

    // Start server
    server.start().await;
}
