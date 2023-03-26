use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FinanceRecord {
    date: String,
    category: String,
    amount: f64,
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
pub fn parse_csv_to_state(csv_data: String) {
    println!("{csv_data}");
    let finance_records = test(csv_data).unwrap();
    println!("{finance_records:?}");
    // todo!()
}
