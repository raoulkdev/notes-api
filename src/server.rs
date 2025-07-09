// Imports
use std::sync::{Arc, Mutex};
use axum::Router;
use axum::routing::get;
use colored::Colorize;
use tokio::net::TcpListener;
use crate::notes::Note;

pub struct Server {
    address: & 'static str,
    notes: Arc<Mutex<Vec<Note>>>
}

impl Server {
    // Create a new server instance
    pub fn new(address: & 'static str) -> Self {
        Self {address, notes: Arc::new(Mutex::new(vec![]))}
    }
    
    // Start server
    pub async fn start(&self) {
        // Bind TCP listener & check if successfull
        let listener = TcpListener::bind(self.address).await;
        match listener { 
            Ok(addr) => {
                println!("{}{}", "Server successfully started on: ".bright_green(), addr.local_addr().unwrap());
                axum::serve(addr, Self::router()).await.unwrap();
            }
            Err(e) => println!("{}{e}", "Could not start server: ".bright_red())
        }
    }
    
    fn router() -> Router {
        Router::new().route("/", get("Hello World"))
    }
}