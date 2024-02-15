// Licensed under the Apache License, Version 2.0 (the "License");
// A copy of the License is located at
// http://www.apache.org/licenses/LICENSE-2.0
//
// This code is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.

#![crate_name = "sample"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]
#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate wasmi;
extern crate wasmi_impl;
extern crate serde_json;

use sgx_types::*;
use std::io::{self, Write};
use std::slice;
use std::vec::Vec;
use core::convert::TryInto;
use serde_json::{json, Value};

/// Executes a WebAssembly (WASM) binary within an SGX enclave, processing JSON data according to a given schema.
/// 
/// # Safety
///
/// This function is marked unsafe because it interacts with raw pointers. The caller must ensure that:
/// - `data_in`, `schema_in`, and `binary` are valid pointers for `data_len`, `schema_len`, and `binary_len` bytes, respectively.
/// - `result_out` points to a sufficiently allocated memory space to store the output.
/// 
/// # Arguments
///
/// * `data_in`: Pointer to the input data bytes.
/// * `data_len`: Length of the input data.
/// * `schema_in`: Pointer to the input schema bytes.
/// * `schema_len`: Length of the input schema.
/// * `binary`: Pointer to the WASM binary.
/// * `binary_len`: Length of the WASM binary.
///
/// # Returns
///
/// Returns an `sgx_status_t` indicating the success or failure of the operation.
#[no_mangle]
pub unsafe extern "C" fn exec_wasm(
    data_in: *const u8,
    data_len: usize,
    schema_in: *const u8,
    schema_len: usize,
    binary: *const u8,
    binary_len: usize,
) -> sgx_status_t {
    // Validate input parameters.
    if binary.is_null() {
        eprintln!("Binary pointer is null.");
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }

    // Initialize a buffer to store the result of WASM execution.
    let mut result_buffer: Vec<u8> = vec![0; 4096];

    // Convert raw pointers to slices for safe access.
    let binary_slice = unsafe { slice::from_raw_parts(binary, binary_len)};
    let data = unsafe { slice::from_raw_parts(data_in, data_len)};
    let schema = unsafe { slice::from_raw_parts(schema_in, schema_len)};

    // Execute the WASM binary with input data and schema, storing the output in result_buffer.
    match wasmi_impl::exec_wasm_with_data_and_schema(
        binary_slice,
        data.as_ptr(),
        data.len(),
        schema.as_ptr(),
        schema.len(),
        result_buffer.as_mut_ptr(),
        result_buffer.len(),
    ) {
        Ok(_) => {
            // Attempt to deserialize the output buffer to JSON.
            match trim_and_deserialize_output(&result_buffer) {
                Ok(json_data) => {
                    println!("\nEnclave Output:\n Deserialized JSON: {:?}\n", json_data);
                    sgx_status_t::SGX_SUCCESS
                },
                Err(e) => {
                    eprintln!("Failed to deserialize JSON: {:?}", e);
                    sgx_status_t::SGX_ERROR_UNEXPECTED // Indicates deserialization failure.
                }
            }
        },
        Err(e) => {
            eprintln!("WASM execution error: {:?}", e);
            sgx_status_t::SGX_ERROR_UNEXPECTED // Indicates execution failure.
        }
    }
}

/// Trims the output buffer at the first null byte and attempts to deserialize it into JSON.
///
/// # Arguments
///
/// * `buffer`: The raw buffer containing the output data.
///
/// # Returns
///
/// Returns a `Result` containing the deserialized JSON `Value` or an error if deserialization fails.
fn trim_and_deserialize_output(buffer: &[u8]) -> Result<Value, serde_json::Error> {
    let valid_data_end = buffer.iter().position(|&x| x == 0x00).unwrap_or(buffer.len());
    let valid_data = &buffer[..valid_data_end];
    serde_json::from_slice(valid_data)
}
