use chat_app_core::create_profile as create_local_profile;
use chat_database::Database;
use serde::Serialize;
use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::Manager;

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
    let database = state.0.lock().map_err(|_| "database lock unavailable")?;
    create_local_profile(&database, &username, &display_name, now_ms()).map_err(|e| e.to_string())
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let data_directory = app.path().app_local_data_dir()?;
            std::fs::create_dir_all(&data_directory)?;
            let database = Database::open(data_directory.join("hearth.sqlite3"))?;
            app.manage(AppDatabase(Mutex::new(database)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![app_state, create_profile])
        .run(tauri::generate_context!())
        .expect("tauri application error");
}
