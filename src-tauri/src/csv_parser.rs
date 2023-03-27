use std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
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

fn vectorize_csv(csv_str: String) -> Result<Vec<FinanceRecord>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());
    let mut finance_records = Vec::new();
    for result in rdr.deserialize() {
        let record: FinanceRecord = result?;
        finance_records.push(record);
    }

    Ok(finance_records)
}

#[tauri::command]
pub fn parse_csv_to_state(name: String, csv_data: String, state: tauri::State<super::State>) {
    let finance_records = vectorize_csv(csv_data).unwrap();
    let user = UserRecords::new(name, finance_records);
    state.0.lock().unwrap().push(user);
    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_to_vec() {
        let csv_str = "date,,category,amount,
2023-3-28,some text,Food,-1,Lunch
2022-9-12,,Income,2,";
        let vectorized_csv = vec![
            FinanceRecord {
                date: "2023-3-28".to_string(),
                category: "Food".to_string(),
                amount: -1.0,
            },
            FinanceRecord {
                date: "2022-9-12".to_string(),
                category: "Income".to_string(),
                amount: 2.0,
            },
        ];
        assert_eq!(vectorize_csv(csv_str.to_string()).unwrap(), vectorized_csv);
    }
}
