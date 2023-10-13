use std::{fs::{self, OpenOptions, File}, io::{Write, Read}};
use serde::{de::DeserializeOwned, Serialize};


pub fn read_data<T: DeserializeOwned>(file_name: &str) -> Option<T> {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut data = String::new();
            if file.read_to_string(&mut data).is_ok() {
                match serde_json::from_str(&data) {
                    Ok(parsed_data) => Some(parsed_data),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn update_data<T: Serialize>(file_name: &str, data: &T) {
    let json = serde_json::to_string_pretty(data).unwrap();

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
    {
        if file.write_all(json.as_bytes()).is_ok() {
            println!("Data updated successfully in {}.", file_name);
        } else {
            eprintln!("Failed to update data in {}.", file_name);
        }
    } else {
        eprintln!("Failed to create or open {} file.", file_name);
    }
}