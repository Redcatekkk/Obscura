#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            obscura_deck::commands::modules_list,
            obscura_deck::commands::sim_status
        ])
        .run(tauri::generate_context!())
    {
        eprintln!("failed to run obscura.deck: {error}");
        std::process::exit(1);
    }
}
