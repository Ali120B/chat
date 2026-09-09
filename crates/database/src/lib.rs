//! Local SQLite persistence. This is the authoritative client history store.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let connection = Connection::open(path)?;
        connection.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        let db = Self { connection };
        db.migrate()?;
        Ok(db)
    }
    pub fn in_memory() -> Result<Self> {
        Self::open(":memory:")
    }
    fn migrate(&self) -> Result<()> {
        self.connection.execute_batch("CREATE TABLE IF NOT EXISTS profiles (id INTEGER PRIMARY KEY CHECK (id = 1), username TEXT NOT NULL, display_name TEXT NOT NULL, created_at_ms INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS conversations (id TEXT PRIMARY KEY, kind TEXT NOT NULL, created_at_ms INTEGER NOT NULL, updated_at_ms INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, conversation_id TEXT NOT NULL REFERENCES conversations(id), sender_device_id TEXT NOT NULL, sequence INTEGER NOT NULL, message_type TEXT NOT NULL, ciphertext BLOB NOT NULL, created_at_ms INTEGER NOT NULL, UNIQUE(conversation_id, sender_device_id, sequence)); CREATE TABLE IF NOT EXISTS pending_outbound (id TEXT PRIMARY KEY, message_id TEXT NOT NULL UNIQUE REFERENCES messages(id), peer_id TEXT NOT NULL, attempt_count INTEGER NOT NULL DEFAULT 0, next_attempt_at_ms INTEGER NOT NULL, state TEXT NOT NULL);")?;
        Ok(())
    }
    pub fn save_profile(&self, username: &str, display_name: &str, now_ms: i64) -> Result<()> {
        self.connection.execute("INSERT INTO profiles (id, username, display_name, created_at_ms) VALUES (1, ?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET username = excluded.username, display_name = excluded.display_name", params![username, display_name, now_ms])?;
        Ok(())
    }
    pub fn profile_exists(&self) -> Result<bool> {
        Ok(self.connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM profiles WHERE id = 1)",
            [],
            |row| row.get::<_, i64>(0),
        )? != 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn migrates_and_persists_profile() {
        let db = Database::in_memory().unwrap();
        assert!(!db.profile_exists().unwrap());
        db.save_profile("ada", "Ada", 1).unwrap();
        assert!(db.profile_exists().unwrap());
    }
}
