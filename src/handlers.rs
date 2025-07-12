// Imports
use crate::notes::{Note, NotePayload};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

// Get all notes
pub async fn get_all_notes(State(notes_db): State<Arc<Pool<Postgres>>>) -> impl IntoResponse {
    let all_notes = sqlx::query_as::<_, Note>("SELECT * FROM notes")
        .fetch_all(&*notes_db)
        .await;
    match all_notes {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{e}"))).into_response(),
    }
}

// Get note by ID
pub async fn get_note_by_id(
    State(notes_db): State<Arc<Pool<Postgres>>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_one(&*notes_db)
        .await;
    match note {
        Ok(n) => (StatusCode::OK, Json(n)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(format!("{e}"))).into_response(),
    }
}

// Add note
pub async fn add_note(
    State(notes_db): State<Arc<Pool<Postgres>>>,
    Json(payload): Json<NotePayload>,
) -> impl IntoResponse {
    let added_note =
        sqlx::query_as::<_, Note>("INSERT INTO notes (title, body) VALUES ($1, $2) RETURNING *")
            .bind(payload.title)
            .bind(payload.body)
            .fetch_one(&*notes_db)
            .await;

    match added_note {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{e}"))).into_response(),
    }
}

// Delete note by ID
pub async fn delete_note_by_id(
    State(notes_db): State<Arc<Pool<Postgres>>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let note_to_delete = sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(&*notes_db)
        .await;
    match note_to_delete {
        Ok(r) => (StatusCode::OK, Json(format!("{:?}", r))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{e}"))).into_response(),
    }
}
