// Imports
use serde::{Deserialize, Serialize};

// Note struct
#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u32,
    title: String,
    body: String
}