use crate::csv_parser::UserRecords;

#[tauri::command]
pub fn get_state(state: tauri::State<super::State>) -> Vec<UserRecords> {
    state.0.lock().unwrap().to_vec()
}
