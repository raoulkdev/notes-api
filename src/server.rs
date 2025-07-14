// Imports
use crate::handlers::{add_note, delete_note_by_id, get_all_notes, get_note_by_id};
use axum::routing::{delete, get, post};
use axum::{Router};
use colored::Colorize;
use sqlx::{Pool, Postgres};
use std::sync::{Arc};
use axum::http::HeaderValue;
use tokio::net::TcpListener;
use tower_http::cors::{Any, Cors, CorsLayer};

// Server struct
pub struct Server {
    address: &'static str,
    notes_db: Arc<Pool<Postgres>>,
}

// Server implementation
impl Server {
    // Create a new server instance
    pub async fn new(address: &'static str, db_url: &'static str) -> Self {
        Self {
            address,
            notes_db: Arc::new(Self::setup_db(db_url).await.unwrap()),
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
                axum::serve(addr, Self::router(self.notes_db.clone()))
                    .await
                    .unwrap();
            }
            Err(e) => println!("{}{e}", "Could not start server: ".bright_red()),
        }
    }

    // Connect to and setup database
    async fn setup_db(db_url: &'static str) -> Result<Pool<Postgres>, String> {
        let pool = sqlx::PgPool::connect(db_url).await;

        match pool {
            Ok(p) => {
                println!("{}{db_url}", "Successfully connected to database at: ".bright_green());

                // Setup database table
                sqlx::query(
                    "
                    CREATE TABLE IF NOT EXISTS notes (
                        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                        title TEXT NOT NULL,
                        body TEXT,
                        created_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    )
                    ",
                )
                .execute(&p)
                .await
                .unwrap();
                Ok(p)
            }
            Err(e) => {
                Err(format!("Could not connect to database at: {db_url}. Error: {e}").to_string())
            }
        }
    }

    // Router
    fn router(notes_db: Arc<Pool<Postgres>>) -> Router {
        Router::new()
            .route("/notes", get(get_all_notes))
            .route("/notes", post(add_note))
            .route("/notes/{capture}", get(get_note_by_id))
            .route("/notes/{capture}", delete(delete_note_by_id))
            .with_state(notes_db)
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                    .allow_methods(Any)
                    .allow_headers(Any)
            )
    }
}
