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
use std::path::Path;
use std::{
    fs::{self, File},
    io::Read,
};

static WASM_FILE_MEDIAN: &str = "get_median_int.wasm";

static WASM_FILE_MEAN: &str = "get_mean_wasm.wasm";

static WASM_FILE_SD: &str = "get_sd_wasm.wasm";

static ENCLAVE_FILE: &str = "enclave.signed.so";

extern "C" {
    fn exec_wasm(eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_in: *const u8,
        data_len: usize,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
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

/// Read data from the JSON file and parse it into a vector of ints.
fn read_data_from_json(file_path: &str, array_name: &str) -> Result<Vec<f32>, Error> {
    let path = Path::new(file_path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json_data: serde_json::Value = serde_json::from_str(&data)?;

    let array_data = json_data[array_name].as_array().unwrap();

    let data_vec: Vec<f32> = array_data
        .iter()
        .map(|v| v.as_f64().unwrap() as f32)
        .collect();

    Ok(data_vec)
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

    //// Mean test
    // Numbers float and integer mixed
     let data_median = read_data_from_json(
        "/root/workspace/wasm-exec-sgx/get-median-wasm/test_data.json",
        "median_int_works_odd",
    )
    .unwrap();
    let serialized_data_median: Vec<u8> = serde_json::to_vec(&data_median).unwrap(); // Create a new byte array that holds the serialized JSON data
    let binary_median = fs::read(WASM_FILE_MEDIAN).unwrap();
    let mut result_out_median = 0f32;

    //// Mean test
    // Numbers float and integer mixed
    let data_mean = read_data_from_json(
        "/root/workspace/wasm-exec-sgx/get-mean-wasm/test_data.json",
        "mean_int_works",
    )
    .unwrap();
    let serialized_data_mean: Vec<u8> = serde_json::to_vec(&data_mean).unwrap(); // Create a new byte array that holds the serialized JSON data
    let binary_mean = fs::read(WASM_FILE_MEAN).unwrap();
    let mut result_out_mean = 0f32;

    // Sd test
    // Numbers float and integer mixed
    let data_sd = read_data_from_json(
        "/root/workspace/wasm-exec-sgx/get-sd-wasm/test_data.json",
        "sd_data",
    )
    .unwrap();
    let serialized_data_sd: Vec<u8> = serde_json::to_vec(&data_sd).unwrap(); // Create a new byte array that holds the serialized JSON data
    let binary_sd = fs::read(WASM_FILE_SD).unwrap();
    let mut result_out_sd = 0f32;
   

    let result = unsafe {
        exec_wasm(
            enclave.geteid(),
            &mut retval,
            serialized_data_median.as_ptr(),
            serialized_data_median.len(),
            binary_median.as_ptr(),
            binary_median.len(),
            &mut result_out_median,
        );
        exec_wasm(
            enclave.geteid(),
            &mut retval,
            serialized_data_mean.as_ptr(),
            serialized_data_mean.len(),
            binary_mean.as_ptr(),
            binary_mean.len(),
            &mut result_out_mean,
        );
        exec_wasm(
            enclave.geteid(),
            &mut retval,
            serialized_data_sd.as_ptr(),
            serialized_data_sd.len(),
            binary_sd.as_ptr(),
            binary_sd.len(),
            &mut result_out_sd,
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
        "[+] ecall_test success, Median result -  {:?}",
        result_out_median
    );

    println!();
    println!(
        "[+] ecall_test success, Mean result -  {:?}",
        result_out_mean
    );

    println!();
    println!(
        "[+] ecall_test success, SD result -  {:?}",
        result_out_sd
    );

    enclave.destroy();
}
