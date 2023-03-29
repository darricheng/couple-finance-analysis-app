#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod csv_parser;
mod get_data;

use std::sync::Mutex;

#[derive(Debug)]
pub struct State(Mutex<Vec<csv_parser::UserRecords>>);

fn main() {
    tauri::Builder::default()
        .manage(State(Mutex::new(vec![])))
        .invoke_handler(tauri::generate_handler![
            csv_parser::parse_csv_to_state,
            get_data::get_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
