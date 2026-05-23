#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let pool = tauri::async_runtime::block_on(obscura_deck::db::connect_memory())
        .expect("failed to initialize local database");

    if let Err(error) = tauri::Builder::default()
        .manage(obscura_deck::commands::DeckState { pool })
        .invoke_handler(tauri::generate_handler![
            obscura_deck::commands::modules_list,
            obscura_deck::commands::modules_set_enabled,
            obscura_deck::commands::modules_trigger_test,
            obscura_deck::commands::logs_recent,
            obscura_deck::commands::sim_status
        ])
        .run(tauri::generate_context!())
    {
        eprintln!("failed to run obscura.deck: {error}");
        std::process::exit(1);
    }
}
