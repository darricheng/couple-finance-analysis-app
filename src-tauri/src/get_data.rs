use serde::{Deserialize, Serialize};

use crate::csv_parser::UserRecords;

#[tauri::command]
pub fn get_state(state: tauri::State<super::State>) -> Vec<UserRecords> {
    state.0.lock().unwrap().to_vec()
}

/**
* Get data by categories
*/
/// This struct will be used to return the data split into categories to the frontend.
#[derive(Debug, Serialize, Deserialize)]
pub struct ByCategories {
    pub category: Vec<String>,
    pub total: Vec<f64>,
}
impl ByCategories {
    fn new(category: Vec<String>, total: Vec<f64>) -> ByCategories {
        ByCategories { category, total }
    }
}

/// This function will return a struct with two keys, "category" and "total".
/// Each key will have a value of a string vector and a float vector respectively.
/// The index of the string vector will correspond to the index of the float vector.
fn sum_by_categories(data: Vec<UserRecords>) -> (Vec<String>, Vec<f64>) {
    let mut categories: Vec<String> = Vec::new();
    let mut totals: Vec<f64> = Vec::new();
    for user in data {
        for record in user.finances {
            if categories.contains(&record.category) {
                let index = categories
                    .iter()
                    .position(|x| x == &record.category)
                    .unwrap();
                totals[index] += record.amount;
            } else {
                categories.push(record.category);
                totals.push(record.amount);
            }
        }
    }

    // Round the totals to 2 decimal places
    for num in totals.iter_mut() {
        *num = (*num * 100.0).round() / 100.0;
    }

    (categories, totals)
}

#[tauri::command]
pub fn get_data_by_categories(state: tauri::State<super::State>) -> ByCategories {
    let data = state.0.lock().unwrap().to_vec();

    let (categories, totals) = sum_by_categories(data);

    ByCategories::new(categories, totals)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::csv_parser::FinanceRecord;

    #[test]
    fn get_totals_by_categories() {
        // Add test case that requires rounding the decimal places
        let data = vec![
            UserRecords::new(
                "Test User".to_string(),
                vec![
                    FinanceRecord::new(
                        NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        "Test Category".to_string(),
                        10.0123,
                    ),
                    FinanceRecord::new(
                        NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        "Test Category".to_string(),
                        10.456,
                    ),
                    FinanceRecord::new(
                        NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        "Test Category 2".to_string(),
                        10.0,
                    ),
                ],
            ),
            UserRecords::new(
                "Test User 2".to_string(),
                vec![
                    FinanceRecord::new(
                        NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        "Test Category".to_string(),
                        10.0,
                    ),
                    FinanceRecord::new(
                        NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                        "Test Category 2".to_string(),
                        10.0,
                    ),
                ],
            ),
        ];

        let (categories, totals) = sum_by_categories(data);

        assert_eq!(categories.len(), 2);
        assert_eq!(totals.len(), 2);
        assert_eq!(categories[0], "Test Category");
        assert_eq!(categories[1], "Test Category 2");
        assert_eq!(totals[0], 30.47);
        assert_eq!(totals[1], 20.0);
    }
}
