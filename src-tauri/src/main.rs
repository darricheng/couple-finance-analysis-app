#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod csv_parser;

use std::sync::Mutex;

#[derive(Debug)]
struct UserRecords {
    name: String,
    finances: Vec<csv_parser::FinanceRecord>,
}

impl UserRecords {
    pub fn new(name: String, finances: Vec<csv_parser::FinanceRecord>) -> UserRecords {
        UserRecords { name, finances }
    }
}

#[derive(Debug)]
pub struct State(Mutex<Vec<UserRecords>>);

fn main() {
    tauri::Builder::default()
        .manage(State(Mutex::new(vec![])))
        .invoke_handler(tauri::generate_handler![csv_parser::parse_csv_to_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
