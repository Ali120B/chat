//! Product-facing state transitions; networking and cryptography will plug in here.
use anyhow::{Result, bail};
use chat_database::Database;

pub fn create_profile(
    database: &Database,
    username: &str,
    display_name: &str,
    now_ms: i64,
) -> Result<()> {
    let normalized = username.trim().to_ascii_lowercase();
    if normalized.len() < 3
        || normalized.len() > 32
        || !normalized
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
    {
        bail!("Username must be 3–32 lowercase letters, numbers, or underscores.");
    }
    if display_name.trim().is_empty() || display_name.chars().count() > 64 {
        bail!("Display name must be between 1 and 64 characters.");
    }
    database.save_profile(&normalized, display_name.trim(), now_ms)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_identity_setup() {
        let db = Database::in_memory().unwrap();
        assert!(create_profile(&db, "Ada", "Ada Lovelace", 0).is_ok());
        assert!(create_profile(&db, "?", "Ada", 0).is_err());
    }
}
