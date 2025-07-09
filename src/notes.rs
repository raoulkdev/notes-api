use std::sync::atomic::{AtomicU32, Ordering};
// Imports
use serde::{Deserialize, Serialize};

// Note struct
#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub body: String,
}

// Note implementation
impl Note {
    // Create new note with unique id
    pub fn new(title: String, body: String) -> Self {
        static ID_COUNTER: AtomicU32 = AtomicU32::new(1);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {id, title, body}
    }
}

// Note payload struct
#[derive(Serialize, Deserialize, Clone)]
pub struct NotePayload {
    pub title: String,
    pub body: String,
}
