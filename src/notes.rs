// Imports
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Note struct
#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_time: chrono::NaiveDateTime,
}

// Note payload struct
#[derive(Serialize, Deserialize, Clone)]
pub struct NotePayload {
    pub title: String,
    pub body: String,
}
