use std::error::Error;

use chrono::NaiveDate;
use csv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FinanceRecord {
    pub date: NaiveDate,
    pub category: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FinanceRecordCsv {
    date: String,
    category: String,
    amount: f64,
}

impl FinanceRecord {
    fn new(date: NaiveDate, category: String, amount: f64) -> FinanceRecord {
        FinanceRecord {
            date,
            category,
            amount,
        }
    }

    pub fn from_csv_record(
        csv_record: FinanceRecordCsv,
        order: &Vec<String>,
        delimeter: &String,
    ) -> FinanceRecord {
        let date_str = &csv_record.date;

        let mut date_numbers: Vec<u32> = Vec::new();
        for num in date_str.split(delimeter) {
            date_numbers.push(num.parse::<u32>().unwrap());
        }

        let mut day = 0;
        let mut month = 0;
        let mut year = 0;

        for (i, num) in date_numbers.iter().enumerate() {
            match order[i].as_str() {
                "d" => {
                    day = *num;
                }
                "m" => {
                    month = *num;
                }
                "y" => {
                    year = *num;
                }
                _ => {
                    panic!("Invalid date format.");
                }
            }
        }

        let date_obj = NaiveDate::from_ymd_opt(year.try_into().unwrap(), month, day).unwrap();

        FinanceRecord::new(date_obj, csv_record.category, csv_record.amount)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct UserRecords {
    name: String,
    pub finances: Vec<FinanceRecord>,
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
    let lowercase_date_format = date_format.to_lowercase();

    let mut d = false;
    let mut m = false;
    let mut y = false;

    let mut delimeter = String::new();

    let mut order: Vec<String> = Vec::new();

    for c in lowercase_date_format.chars() {
        match c {
            'd' => {
                if d {
                    continue;
                }
                d = true;
                order.push("d".to_string());
            }
            'm' => {
                if m {
                    continue;
                }
                m = true;
                order.push("m".to_string());
            }
            'y' => {
                if y {
                    continue;
                }
                y = true;
                order.push("y".to_string());
            }
            _ => {
                delimeter = c.to_string();
            }
        }
    }

    let mut finance_records: Vec<FinanceRecord> = Vec::new();

    // Convert the string dates to NaiveDates.
    for record in string_date_csv.into_iter() {
        finance_records.push(FinanceRecord::from_csv_record(record, &order, &delimeter));
    }

    finance_records
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

    println!("Users: {:?}", users_vec);
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
