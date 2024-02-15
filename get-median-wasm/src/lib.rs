
#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;
use serde_json::{Value, json};

/// Calculates the median of an array of numbers.
/// Numbers could be floats or integers.
/// Return a pointer to an array of results in linear memory.
#[no_mangle]
pub unsafe extern "C" fn exec(data: *const u8, len: u32) -> *const f32 {
    let data_slice = unsafe { slice::from_raw_parts(data, len as usize) };
    let json_data: Value = serde_json::from_slice(data_slice).unwrap();
    let columns = json_data.as_object().unwrap();

    let mut results = Vec::new();
    for (_column_name, values) in columns {
        let mut numbers: Vec<f32> = values.as_array().unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect();

        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if numbers.len() % 2 == 0 {
            let mid = numbers.len() / 2;
            (numbers[mid - 1] + numbers[mid]) / 2.0
        } else {
            numbers[numbers.len() / 2]
        };
        results.push(median);
    }

    // Allocate memory in the Wasm module's linear memory to store the results.
    let result_ptr = results.as_ptr();
    std::mem::forget(results); // Prevent deallocation
    result_ptr
}

// Tests and other module definitions...
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::fs;
    use std::path::PathBuf;

    /// Function to read the JSON file and serialize the data
    fn read_and_serialize_json(file_name: &str) -> Vec<u8> {
        // Construct the path to the JSON file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        println!("{:?}", &path);
        path.push(file_name);
        println!("{:?}", &path);
        // Read the JSON file
        let json_str = fs::read_to_string(path)
            .expect("Failed to read JSON file");
        println!("{:?}", &json_str);
        // Serialize the JSON string into bytes
        json_str.into_bytes()
    }

    #[test]
        #[test]
    fn test_wasm_binary() {
        let json_data = read_and_serialize_json("test.json");

        // Directly use the exec function
        let result_ptr = unsafe { exec(json_data.as_ptr(), json_data.len() as u32) };

        let results = unsafe { std::slice::from_raw_parts(result_ptr, 2) };
        println!("results from test : {:?}", &results);
        // Expected median values for Column_1 and Column_2
        let expected_median_1 = 6.1; // Median of [1, 3, 3, 6.1, 7, 8.1, 9]
        let expected_median_2 = 7.0; // Median of [3, 5, 6.1, 7, 7, 8.1, 9]

        assert!((results[0] - expected_median_1).abs() < f32::EPSILON);
        assert!((results[1] - expected_median_2).abs() < f32::EPSILON);
    }

}
