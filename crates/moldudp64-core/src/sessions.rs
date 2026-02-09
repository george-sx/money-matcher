use crate::SessionTable;
use std::collections::HashMap;

impl SessionTable {
    pub fn new() -> Self {
        SessionTable {
            sessions: HashMap::new(),
        }
    }

    pub fn generate_session_id(&mut self) -> [u8; 10] {
        let s = format!("MM{:08}", self.sessions.len() + 1);
        let mut session_id = [b' '; 10];

        session_id.copy_from_slice(s.as_bytes());
        session_id
    }

    pub fn add_session(&mut self, session_id: [u8; 10], sequence_number: [u8; 8]) {
        self.sessions.insert(session_id, sequence_number);
    }

    pub fn get_sequence_number(&self, session_id: &[u8; 10]) -> Option<&[u8; 8]> {
        self.sessions.get(session_id)
    }

    pub fn remove_session(&mut self, session_id: &[u8; 10]) {
        self.sessions.remove(session_id);
    }
}
