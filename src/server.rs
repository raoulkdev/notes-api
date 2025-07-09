use std::sync::{Arc, Mutex};
// Imports
use crate::note::Note;

pub struct Server {
    address: & 'static str,
    notes: Arc<Mutex<Vec<Note>>>
}

impl Server {
    pub fn new(address: & 'static str) -> Self {
        Self {address, notes: Arc::new(Mutex::new(vec![]))}
    }
}