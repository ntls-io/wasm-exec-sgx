// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

extern crate sgx_types;
extern crate sgx_urts;
extern crate wabt;
use serde_json::Error;
use sgx_types::*;
use sgx_urts::SgxEnclave;

extern crate serde_json;
use serde_json::json;
use std::path::Path;
use std::{
    fs::{self, File},
    io::Read,
};
use std::env;
use std::path::PathBuf;
use serde_json::Value;

static WASM_FILE_MEDIAN: &str = "get_median_wasm.wasm";

static WASM_FILE_MEAN: &str = "get_mean_wasm.wasm";

static WASM_FILE_SD: &str = "get_sd_wasm.wasm";

static ENCLAVE_FILE: &str = "enclave.signed.so";

extern "C" {
    fn exec_wasm(eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_in: *const u8,
        data_len: usize,
        schema_in: *const u8,
        schema_len: usize,
        binary: *const u8,
        binary_len: usize,
    ) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}




fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let binary_median = fs::read(WASM_FILE_MEAN).unwrap();
 
    // Construct the path to the JSON data and schema files.
    let test_data_file_path = env::current_dir().unwrap().join("..").join("test_data").join("1_test_data.json");
    let test_schema_file_path = env::current_dir().unwrap().join("..").join("test_data").join("1_test_schema.json");

    // Read the JSON data and schema from their respective files.
    let test_json_data = read_json_from_file(&test_data_file_path).expect("Error reading JSON data file");
    let test_json_schema = read_json_from_file(&test_schema_file_path).expect("Error reading JSON schema file");

    // Serialize the JSON data and schema.
    let test_serialized_data = serde_json::to_vec(&test_json_data).expect("Failed to serialize data");
    let test_serialized_schema = serde_json::to_vec(&test_json_schema).expect("Failed to serialize schema");

    let result = unsafe {
        exec_wasm(
            enclave.geteid(),
            &mut retval,
            test_serialized_data.as_ptr(),
            test_serialized_data.len(),
            test_serialized_schema.as_ptr(),
            test_serialized_schema.len(),
            binary_median.as_ptr(),
            binary_median.len(),
        )
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {}
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!(
        "[+] ecall_test success",
        
    );

    enclave.destroy();
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, serde_json::Error> {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    serde_json::from_str(&contents)
}
