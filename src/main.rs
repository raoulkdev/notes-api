// Module Declarations
mod handlers;
mod notes;
mod server;

// Imports
use crate::server::Server;

#[tokio::main]
async fn main() {
    // Create a new server
    let server = Server::new("0.0.0.0:3000", "postgresql://raoulkaleba:oJTPhj3i91x2kzI6N7xssYevu25X3BcF@dpg-d1pa8b2dbo4c7388bhjg-a.oregon-postgres.render.com/notesdb_y3fl?sslmode=require").await;

    // Start server
    server.start().await;
}
