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
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::fs;

static WASM_FILE_MEDIAN_INT: &str = "get_median_int.wasm";
static WASM_FILE_MEDIAN_FLOAT: &str = "get_median_float.wasm";

static WASM_FILE_MEAN_INT: &str = "get_mean_int.wasm";
static WASM_FILE_MEAN_FLOAT: &str = "get_mean_float.wasm";

static WASM_FILE_SD_INT: &str = "get_sd_int.wasm";
static WASM_FILE_SD_FLOAT: &str = "get_sd_float.wasm";

static WASM_FILE_APPEND: &str = "wasm_append.wasm";

static ENCLAVE_FILE: &str = "enclave.signed.so";

extern "C" {

    fn exec_wasm_median_int(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut i32,
    ) -> sgx_status_t;

    fn exec_wasm_median_float(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

    fn exec_wasm_mean_int(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut i32,
    ) -> sgx_status_t;

    fn exec_wasm_mean_float(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

    fn exec_wasm_sd_int(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

    fn exec_wasm_sd_float(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

    fn exec_wasm_append(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut i32,
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

    let binary_median_int = fs::read(WASM_FILE_MEDIAN_INT).unwrap();
    let binary_median_float = fs::read(WASM_FILE_MEDIAN_FLOAT).unwrap();

    let binary_mean_int = fs::read(WASM_FILE_MEAN_INT).unwrap();
    let binary_mean_float = fs::read(WASM_FILE_MEAN_FLOAT).unwrap();

    let binary_sd_int = fs::read(WASM_FILE_SD_INT).unwrap();
    let binary_sd_float = fs::read(WASM_FILE_SD_FLOAT).unwrap();

    let binary_wasm_append = fs::read(WASM_FILE_APPEND).unwrap();

    let mut result_out_median_int = 0i32;
    let mut result_out_median_float = 0f32;

    let mut result_out_mean_int = 0i32;
    let mut result_out_mean_float = 0f32;

    let mut result_out_sd_int = 0f32;
    let mut result_out_sd_float = 0f32;

    let mut result_append_out = 0i32;

    let result = unsafe {
        exec_wasm_median_int(
            enclave.geteid(),
            &mut retval,
            binary_median_int.as_ptr(),
            binary_median_int.len(),
            &mut result_out_median_int,
        );

        exec_wasm_median_float(
            enclave.geteid(),
            &mut retval,
            binary_median_float.as_ptr(),
            binary_median_float.len(),
            &mut result_out_median_float,
        );

        exec_wasm_mean_int(
            enclave.geteid(),
            &mut retval,
            binary_mean_int.as_ptr(),
            binary_mean_int.len(),
            &mut result_out_mean_int,
        );

        exec_wasm_mean_float(
            enclave.geteid(),
            &mut retval,
            binary_mean_float.as_ptr(),
            binary_mean_float.len(),
            &mut result_out_mean_float,
        );

        exec_wasm_sd_int(
            enclave.geteid(),
            &mut retval,
            binary_sd_int.as_ptr(),
            binary_sd_int.len(),
            &mut result_out_sd_int,
        );

        exec_wasm_sd_float(
            enclave.geteid(),
            &mut retval,
            binary_sd_float.as_ptr(),
            binary_sd_float.len(),
            &mut result_out_sd_float,
        );

        exec_wasm_append(
            enclave.geteid(),
            &mut retval,
            binary_wasm_append.as_ptr(),
            binary_wasm_append.len(),
            &mut result_append_out,
        )
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {}
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!("[+] ecall_test success, Median Int result -  {:?}", result_out_median_int);
    println!("[+] ecall_test success, Median Float result -  {:?}", result_out_median_float);
    println!();
    println!("[+] ecall_test success, Mean Int result -  {:?}", result_out_mean_int);
    println!("[+] ecall_test success, Mean Float result -  {:?}", result_out_mean_float);
    println!();
    println!("[+] ecall_test success, SD Int result -  {:?}", result_out_sd_int);
    println!("[+] ecall_test success, SD Float result -  {:?}", result_out_sd_float);
    println!();
    println!("[+] ecall_test success, WASM Append successful -  {:?}", result_append_out);

    enclave.destroy();
}
