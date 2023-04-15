use std::error::Error;

use chrono::NaiveDate;
use csv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FinanceRecord {
    date: NaiveDate,
    category: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct FinanceRecordCsv {
    date: String,
    category: String,
    amount: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserRecords {
    name: String,
    finances: Vec<FinanceRecord>,
}

impl UserRecords {
    pub fn new(name: String, finances: Vec<FinanceRecord>) -> UserRecords {
        UserRecords { name, finances }
    }
}

fn vectorize_csv(csv_str: String) -> Result<Vec<FinanceRecordCsv>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());
    let mut finance_records = Vec::new();
    for result in rdr.deserialize() {
        let record: FinanceRecordCsv = result?;
        finance_records.push(record);
    }

    Ok(finance_records)
}

/// Format the provided dates in the csv data into a common NaiveDate format.
fn format_csv_date(
    string_date_csv: Vec<FinanceRecordCsv>,
    date_format: String,
) -> Vec<FinanceRecord> {
    todo!()
}

/// user_date_format will be the string entered by the user indicating the format of their dates.
/// For example, if the user enters "dd-mm-yyyy", then the string will be parsed so that the
/// respective data is used to create the NaiveDates for the individual records.
#[tauri::command]
pub fn parse_csv_to_state(
    name: String,
    csv_data: String,
    user_date_format: String,
    state: tauri::State<super::State>,
) -> Result<Vec<String>, String> {
    let finance_records_string_date = vectorize_csv(csv_data).map_err(|e| e.to_string())?;

    let finance_records = format_csv_date(finance_records_string_date, user_date_format);

    let data = UserRecords::new(name, finance_records);
    let mut users_vec = state.0.lock().unwrap();
    users_vec.push(data);

    let mut names = Vec::new();
    for user in users_vec.iter() {
        names.push(user.name.clone())
    }
    Ok(names)
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
            FinanceRecordCsv {
                date: "2023-3-28".to_string(),
                category: "Food".to_string(),
                amount: -1.0,
            },
            FinanceRecordCsv {
                date: "2022-9-12".to_string(),
                category: "Income".to_string(),
                amount: 2.0,
            },
        ];
        assert_eq!(vectorize_csv(csv_str.to_string()).unwrap(), vectorized_csv);
    }
}
