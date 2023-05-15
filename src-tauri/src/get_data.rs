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
#[tauri::command]
pub fn get_data_by_categories(state: tauri::State<super::State>) -> ByCategories {
    let mut categories: Vec<String> = Vec::new();
    let mut totals: Vec<f64> = Vec::new();

    let data = state.0.lock().unwrap().to_vec();

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

    // ! NOT TESTED YET
    todo!();
    ByCategories::new(categories, totals)
}
