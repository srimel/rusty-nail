use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Iterates over the CSV file and populates a HashMap.
/// The first line of the CSV defines the keys for the HashMap.
/// For each key in the HashMap, a Vec is created to store the values.
/// The HashMap is returned.
pub fn read_csv_file(file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the first line (header)
    let header_line = match reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return Err("CSV file is empty or has no header.".into()),
    };

    // Parse the header line
    let headers: Vec<&str> = header_line.split(',').collect();

    // Create the HashMap
    let mut data_map: HashMap<String, Vec<String>> = HashMap::new();

    // Initialize the HashMap values
    for header in &headers {
        data_map.insert(header.to_string(), Vec::new());
    }

    // Read the remaining lines (data)
    for line in BufReader::new(File::open(file_path)?).lines().skip(1) {
        let data_line = line?;
        let values: Vec<&str> = data_line.split(',').collect();

        // Insert values into corresponding HashMap key
        for (i, value) in values.iter().enumerate() {
            if let Some(key) = headers.get(i) {
                data_map.get_mut(*key).unwrap().push(value.to_string());
            }
        }
    }

    // Return the populated HashMap
    Ok(data_map)
}
