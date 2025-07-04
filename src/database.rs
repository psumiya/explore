use rusqlite::{Connection, Result, params};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory()?));
        Ok(Db { conn })
    }

    pub fn init_table(&self) -> Result<usize> {
        self.conn.lock().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )
    }

    pub fn insert_user(&self, id: i64, name: &str) -> Result<usize> {
        self.conn.lock().unwrap().execute(
            "INSERT INTO users (id, name) VALUES (?1, ?2)",
            params![id, name],
        )
    }

    pub fn get_all_users(&self) -> Result<Vec<(i64, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name FROM users")?;
        let user_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut users = Vec::new();
        for user in user_iter {
            users.push(user?);
        }
        Ok(users)
    }

    pub fn get_user_by_id(&self, id: i64) -> Result<Option<(i64, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name FROM users WHERE id = ?1")?;
        let mut user_iter = stmt.query_map(params![id], |row| Ok((row.get(0)?, row.get(1)?)))?;

        if let Some(user) = user_iter.next() {
            Ok(Some(user?))
        } else {
            Ok(None)
        }
    }

    pub fn update_user(&self, id: i64, name: &str) -> Result<usize> {
        self.conn.lock().unwrap().execute(
            "UPDATE users SET name = ?1 WHERE id = ?2",
            params![name, id],
        )
    }

    pub fn delete_user(&self, id: i64) -> Result<usize> {
        self.conn.lock().unwrap().execute(
            "DELETE FROM users WHERE id = ?1",
            params![id],
        )
    }
}
