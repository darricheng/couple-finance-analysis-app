use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FinanceRecord {
    date: String,
    category: String,
    amount: f64,
}

#[derive(Debug)]
pub struct UserRecords {
    name: String,
    finances: Vec<FinanceRecord>,
}

impl UserRecords {
    pub fn new(name: String, finances: Vec<FinanceRecord>) -> UserRecords {
        UserRecords { name, finances }
    }
}

fn test(foo: String) -> Result<Vec<FinanceRecord>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(foo.as_bytes());
    let mut finance_records = Vec::new();
    for result in rdr.deserialize() {
        let record: FinanceRecord = result?;
        finance_records.push(record);
    }

    Ok(finance_records)
}

#[tauri::command]
pub fn parse_csv_to_state(name: String, csv_data: String, state: tauri::State<super::State>) {
    let finance_records = test(csv_data).unwrap();
    let user = UserRecords::new(name, finance_records);
    state.0.lock().unwrap().push(user);
    // todo!()
}
