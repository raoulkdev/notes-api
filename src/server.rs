// Imports
use crate::notes::{Note, NotePayload};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use colored::Colorize;
use std::sync::{Arc, Mutex};
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
                axum::serve(addr, Self::router(self.notes.clone()))
                    .await
                    .unwrap();
            }
            Err(e) => println!("{}{e}", "Could not start server: ".bright_red()),
        }
    }

    // Router
    fn router(notes: Arc<Mutex<Vec<Note>>>) -> Router {
        Router::new()
            .route("/notes", get(Self::get_all_notes))
            .route("/notes", post(Self::add_note))
            .route("/notes/{capture}", get(Self::get_note_by_id))
            .route("/notes/{capture}", delete(Self::delete_note_by_id))
            .with_state(notes)
    }

    // Add note
    async fn add_note(
        State(notes): State<Arc<Mutex<Vec<Note>>>>,
        Json(payload): Json<NotePayload>,
    ) -> impl IntoResponse {
        let new_note = Note::new(payload.title, payload.body);
        notes.lock().unwrap().push(new_note.clone());
        (StatusCode::CREATED, Json(new_note))
    }

    // Get all notes
    async fn get_all_notes(State(notes): State<Arc<Mutex<Vec<Note>>>>) -> impl IntoResponse {
        (StatusCode::OK, Json(notes.lock().unwrap().clone()))
    }

    // Get note by ID
    async fn get_note_by_id(
        State(notes): State<Arc<Mutex<Vec<Note>>>>,
        Path(id): Path<u32>,
    ) -> impl IntoResponse {
        match notes.lock().unwrap().clone().iter().find(|n| n.id == id) {
            Some(n) => (StatusCode::FOUND, Json(n)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(format!("Could not find note with id: {id}")),
            )
                .into_response(),
        }
    }

    // Delete note by ID
    async fn delete_note_by_id(
        State(notes): State<Arc<Mutex<Vec<Note>>>>,
        Path(id): Path<u32>,
    ) -> impl IntoResponse {
        match notes
            .lock()
            .unwrap()
            .clone()
            .iter()
            .position(|n| n.id == id)
        {
            Some(n) => {
                notes.lock().unwrap().remove(n);
                (StatusCode::NO_CONTENT, Json(n)).into_response()
            }
            None => (
                StatusCode::NOT_FOUND,
                Json(format!("Could not find note with id: {id}")),
            )
                .into_response(),
        }
    }
}
