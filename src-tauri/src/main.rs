#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data_parser;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            data_parser::csv_parser::parse_csv_to_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
