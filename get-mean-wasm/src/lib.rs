// Disable the standard library to ensure compatibility with WebAssembly and embedded environments.
#![no_std]
// Enable strict linting rules to prevent memory leaks and ensure safety in unsafe blocks.
#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]

// Import necessary modules for JSON processing and memory allocation in a no_std environment.
use serde_json::{Value, json, from_slice, to_vec}; // For working with JSON data.
extern crate alloc; // To use Vec and other collection types.
use core::slice; // For operations on slices.
use core::mem; // For memory-related operations.
use alloc::vec::Vec; // To use Vec in no_std environment.


/// This function calculates averages of numeric columns in a JSON dataset according to a JSON schema and writes the results back as JSON.
///
/// # Arguments
///
/// * `input_data_ptr`: A raw pointer to the serialized JSON data.
/// * `input_data_len`: The length of the serialized JSON data.
/// * `input_schema_ptr`: A raw pointer to the serialized JSON schema.
/// * `input_schema_len`: The length of the serialized JSON schema.
/// * `output_ptr`: A raw pointer to the memory where the output (serialized JSON with averages) should be written.
/// * `output_len`: The maximum length of the output buffer.
///
/// # Safety
///
/// This function is unsafe because it directly manipulates raw pointers and performs unchecked operations on them. 
/// The caller must ensure that all pointers are valid and point to sufficiently allocated memory spaces.
#[no_mangle]
pub unsafe extern "C" fn exec(input_data_ptr: *const u8, input_data_len: usize, input_schema_ptr: *const u8, input_schema_len: usize, output_ptr: *mut u8, output_len: usize) {
    // Deserialize the input JSON data and schema.
    let data_slice = unsafe { slice::from_raw_parts(input_data_ptr, input_data_len)} ;
    let schema_slice = unsafe {slice::from_raw_parts(input_schema_ptr, input_schema_len)};
    let data: Value = from_slice(data_slice).expect("Failed to deserialize data");
    let schema: Value = from_slice(schema_slice).expect("Failed to deserialize schema");

    // Initialize an empty map to hold the computation results.
    let mut result = serde_json::Map::new();

    // Iterate through the schema, compute averages for each numeric column specified, and populate the result map.
    for (key, schema_details) in schema["properties"].as_object().unwrap() {
        if let Some(column_data) = data.get(key) {
            if schema_details["type"] == "array" && schema_details["items"]["type"] == "number" {
                let numbers: Vec<f32> = column_data.as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect();
                let average = numbers.iter().sum::<f32>() / numbers.len() as f32;
                result.insert(key.clone(), json!({"Average": average}));
            }
        }
    }

    // Serialize the result map to JSON and write it to the specified output buffer.
    let serialized_result = to_vec(&result).expect("Failed to serialize result");
    assert!(output_len >= serialized_result.len(), "Output buffer too small");
    let output_slice = unsafe {slice::from_raw_parts_mut(output_ptr, serialized_result.len())};
    output_slice.copy_from_slice(&serialized_result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::mem;
    use serde_json::{Value, json, from_slice, to_vec}; // For working with JSON data.

    fn trim_and_deserialize_output(buffer: &[u8]) -> Result<serde_json::Value, serde_json::Error> {
    // Search for the first null byte which indicates the end of the valid data
    let valid_data_end = buffer.iter().position(|&x| x == 0x00).unwrap_or_else(|| buffer.len());

    // Create a slice of the buffer up to the found position
    let valid_data = &buffer[..valid_data_end];

    // Deserialize the JSON data from the trimmed buffer
    serde_json::from_slice(valid_data)
}

    #[test]
    fn test_exec_function() {
        // Prepare test data and schema as JSON

        // Serialize the JSON data to bytes as would be done before passing to the WASM module
        let numbers: [f32; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let test_bytes: Vec<u8> = numbers.iter()
            .flat_map(|&number| number.to_le_bytes().to_vec())
            .collect();
        println!("{:?}", &test_bytes);

        // Create a buffer for the output, large enough to hold the result string
        let mut output_buffer = vec![0u8; 1024];

        // Call the exec function safely within a test
        unsafe {
            exec(
                test_bytes.as_ptr(),
                test_bytes.len(),
                output_buffer.as_mut_ptr(),
            );
        }

        // Convert output bytes back to a string
        let result_message = String::from_utf8(output_buffer)
            .expect("Failed to convert output bytes to string")
            .trim_matches(char::from(0))
            .to_string();

        // Print the result for verification
        println!("Result message: {}", result_message);

        // Verify the result (this part is dependent on what you expect as output, 
        // here we just check if the string contains the expected message part)
        assert!(result_message.contains("this is the result of the computation"));
    }

    #[test]
    fn test_exec_two_function() {
        let json_data = json!({
            "Column_1": [8.1, 6.1, 3, 3, 7, 1, 9],
            "Column_2": [8.1, 6.1, 5, 3, 7, 7, 9],
            "Column_3": [2,5,5,5,5,5,6,7,4,4,4,4,4],
        });

        let schema = json!({
            "Column_1": "f32",
            "Column_2": "f32",
            "Column_3": "f32"
        });

        let serialized_data = serde_json::to_vec(&json_data).unwrap();
        let serialized_schema = serde_json::to_vec(&schema).unwrap();

        let mut output_buffer = vec![0u8; 1024]; // Adjust size as needed

        unsafe {
            exec_two(
                serialized_data.as_ptr(),
                serialized_data.len(),
                serialized_schema.as_ptr(),
                serialized_schema.len(),
                output_buffer.as_mut_ptr(),
                output_buffer.len(),
            );
        }

    // Trim and deserialize output buffer
    let deserialized_json_data = match trim_and_deserialize_output(&output_buffer) {
        Ok(json_data) => {
            println!("Deserialized JSON: {:?}", json_data);
            json_data // Here we save the deserialized data to use later
        },
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {:?}", e);
            return; // If deserialization fails, return early from the test
        }
    };

    // Now, you can correctly assert using deserialized_json_data
    assert!(deserialized_json_data.get("Column_1").unwrap().get("Average").is_some());
    assert!(deserialized_json_data.get("Column_2").unwrap().get("Average").is_some());
}

}

