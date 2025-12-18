use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new() -> Self {
        let conn = Connection::open("users.db").unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS users (\
        id INTEGER PRIMARY KEY,\
        name TEXT NOT NULL,\
        age INTEGER NOT NULL)",
                     [],
        ).unwrap();

        AppState {
            db: Arc::new(Mutex::new(conn)),
        }
    }
}