use chat_app_core::create_profile;
use chat_database::Database;
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

struct AppDatabase(Mutex<Database>);
#[derive(Serialize)]
struct AppState {
    profile_exists: bool,
}
fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
#[tauri::command]
fn app_state(state: tauri::State<AppDatabase>) -> Result<AppState, String> {
    Ok(AppState {
        profile_exists: state
            .0
            .lock()
            .map_err(|_| "database lock unavailable")?
            .profile_exists()
            .map_err(|e| e.to_string())?,
    })
}
#[tauri::command]
fn create_profile(
    username: String,
    display_name: String,
    state: tauri::State<AppDatabase>,
) -> Result<(), String> {
    create_profile(
        &state.0.lock().map_err(|_| "database lock unavailable")?,
        &username,
        &display_name,
        now_ms(),
    )
    .map_err(|e| e.to_string())
}
fn main() {
    let database_path = PathBuf::from("hearth.sqlite3");
    let database = Database::open(database_path).expect("local database must open");
    tauri::Builder::default()
        .manage(AppDatabase(Mutex::new(database)))
        .invoke_handler(tauri::generate_handler![app_state, create_profile])
        .run(tauri::generate_context!())
        .expect("tauri application error");
}
