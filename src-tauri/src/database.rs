use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("app.db")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn init(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_item(&self, content: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("INSERT INTO items (content) VALUES (?1)", [content])?;
        Ok(())
    }

    pub fn get_items(&self) -> Result<Vec<(i64, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, content FROM items")?;
        let items = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        items.collect()
    }
}
