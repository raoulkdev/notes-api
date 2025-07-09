// Imports
use crate::notes::{Note, NotePayload};
use axum::{Json, Router};
use axum::routing::{get, post};
use colored::Colorize;
use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::net::TcpListener;

// Server struct
pub struct Server {
    address: &'static str,
    notes: Arc<Mutex<Vec<Note>>>,
}

// Server implementation
impl Server {
    // Create a new server instance
    pub fn new(address: &'static str) -> Self {
        Self {
            address,
            notes: Arc::new(Mutex::new(vec![])),
        }
    }

    // Start server
    pub async fn start(&self) {
        // Bind TCP listener & check if successful
        let listener = TcpListener::bind(self.address).await;
        match listener {
            Ok(addr) => {
                println!(
                    "{}{}",
                    "Server successfully started on: ".bright_green(),
                    addr.local_addr().unwrap()
                );
                axum::serve(addr, Self::router(self.notes.clone())).await.unwrap();
            }
            Err(e) => println!("{}{e}", "Could not start server: ".bright_red()),
        }
    }

    // Router
    fn router(notes: Arc<Mutex<Vec<Note>>>) -> Router {
        Router::new().route("/", post(Self::add_note)).with_state(notes)
    }
    
    // Add note
    async fn add_note(State(notes): State<Arc<Mutex<Vec<Note>>>>, Json(payload): Json<NotePayload>) -> impl IntoResponse {
        let new_note = Note::new(payload.title, payload.body);
        notes.lock().unwrap().push(new_note.clone());
        (StatusCode::CREATED, Json(new_note))
    }
}
