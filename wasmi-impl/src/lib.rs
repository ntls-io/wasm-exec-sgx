#![cfg_attr(not(test), no_std)]
extern crate alloc;
use wasmi::{
    self, Error as WasmiError,memory_units::Pages, ExternVal, ImportsBuilder, MemoryInstance, ModuleInstance,
    NopExternals, RuntimeValue,
};
use wasmi::FuncInstance;
use wasmi::Signature;
use wasmi::ValueType;
use wasmi::ModuleImportResolver;
use wasmi::MemoryRef;
use alloc::string::ToString;
use core::convert::TryInto;
use alloc::vec::Vec;
use alloc::vec;
use core::slice;

static ENTRYPOINT: &str = "exec";

#[derive(Debug)]
pub enum ExecWasmError {
    WasmiError(WasmiError),
    ExecutionError,
    MemoryError,
}

impl From<wasmi::Error> for ExecWasmError {
    fn from(err: wasmi::Error) -> Self {
        Self::WasmiError(err)
    }
}

// This struct will serve as our custom import resolver.
struct CustomImportResolver {
    memory: MemoryRef,
}
impl ModuleImportResolver for CustomImportResolver {
    fn resolve_memory(
        &self,
        field_name: &str,
        _memory_type: &wasmi::MemoryDescriptor,
    ) -> Result<MemoryRef, wasmi::Error> {
        if field_name == "memory" {
            Ok(self.memory.clone())
        } else {
            Err(wasmi::Error::Instantiation(
                "No such memory: {}".to_string()),
            )
        }
    }
}

pub fn exec_wasm_with_data(
    binary: &[u8],
    data: &[u8],
    result_buffer: &mut [u8],
) -> Result<(), ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;

    // TODO: Calculate the memory size always be larger than `data`
    let mem_instance = MemoryInstance::alloc(Pages(100), None)?;

    // TODO: Error Handling
    // mem_instance.set(0, data).map_err(|_| ExecWasmError::MemoryError)?;
    // Adjust the offset for input and output pointers.
    let input_offset = 0; // Starting point for the input in memory.
    let result_buffer_offset = data.len() as u32 + 4; // Reserve space after input data; ensure it does not overlap.

    // Write input data to memory.
    mem_instance.set(input_offset, data)?;

    // let externals = FuncInstance::alloc_host(
    //     Signature::new(&[ValueType::I32][..], Some(ValueType::I32)),
    //     0,
    // );

    let resolver = CustomImportResolver {
        memory: mem_instance.clone(),
    };

    // let imports = [ExternVal::Memory(mem_instance)];

    let instance = ModuleInstance::new(&module, &ImportsBuilder::new().with_resolver("env", &resolver))
        .map_err(|_| ExecWasmError::ExecutionError)?
        .assert_no_start();

    let params = [
        RuntimeValue::I32(input_offset as i32),
        RuntimeValue::I32(data.len() as i32),
        RuntimeValue::I32(result_buffer_offset as i32),
       // RuntimeValue::I32(output_buffer_size as i32)
    ];

    // Call the `exec` function.
    instance.invoke_export(ENTRYPOINT, &params, &mut NopExternals)
        .map_err(|_| ExecWasmError::ExecutionError)?;

    // Read the result from memory.
    mem_instance.get_into((result_buffer_offset as usize).try_into().unwrap(), result_buffer)
        .map_err(|_| ExecWasmError::MemoryError)?;

    Ok(())
}

pub fn exec_wasm(binary: &[u8]) -> Result<Option<RuntimeValue>, ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;
    // TODO: This panics. We probably want to run start functions
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())?.assert_no_start();
    Ok(instance.invoke_export(ENTRYPOINT, &[], &mut NopExternals)?)
}

pub fn exec_wasm_with_data_and_schema(
    binary: &[u8],
    input_data_ptr: *const u8,
    input_data_len: usize,
    input_schema_ptr: *const u8,
    input_schema_len: usize,
    output_ptr: *mut u8,
    output_len: usize,
) -> Result<(), ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;

    // Allocate memory large enough to accommodate data, schema, and output
    let mem_instance = MemoryInstance::alloc(Pages(100), None)?;

    // Write input data and schema to WASM module's memory
    let data_slice = unsafe { slice::from_raw_parts(input_data_ptr, input_data_len) };
    let schema_slice = unsafe { slice::from_raw_parts(input_schema_ptr, input_schema_len) };

    let input_offset = 0;
    mem_instance.set(input_offset, data_slice)?;

    let schema_offset = input_data_len as u32; // Adjust offset for schema data
    mem_instance.set(schema_offset, schema_slice)?;

    let result_buffer_offset = schema_offset + input_schema_len as u32; // Adjust offset for result buffer

    let resolver = CustomImportResolver {
        memory: mem_instance.clone(),
    };

    let instance = ModuleInstance::new(&module, &ImportsBuilder::new().with_resolver("env", &resolver))
        .map_err(|_| ExecWasmError::ExecutionError)?
        .assert_no_start();

    // Adjust parameters to pass to WASM function
    let params = [
        RuntimeValue::I32(input_offset as i32),
        RuntimeValue::I32(input_data_len as i32),
        RuntimeValue::I32(schema_offset as i32),
        RuntimeValue::I32(input_schema_len as i32),
        RuntimeValue::I32(result_buffer_offset as i32),
        RuntimeValue::I32(output_len as i32),
    ];

    instance.invoke_export(ENTRYPOINT, &params, &mut NopExternals)
        .map_err(|_| ExecWasmError::ExecutionError)?;

    // Read the result from memory into the provided output buffer
    let output_slice = unsafe { slice::from_raw_parts_mut(output_ptr, output_len) };
    mem_instance.get_into((result_buffer_offset as usize).try_into().unwrap(), output_slice)
        .map_err(|_| ExecWasmError::MemoryError)?;
    Ok(())
}



#[cfg(test)]
mod tests {

    use super::*;
    use wabt;

    #[test]
    fn exec_wasm_works() {
        // let wasm_binary = wabt::wat2wasm(
        //     r#"
        //     (module
        //         (import "env" "memory" (memory 1))
        //         (func $exec (export "exec")
        //             (param $input_ptr i32) (param $input_len i32) (param $output_ptr i32)
        //             ;; Example: Write the value 42 to the location pointed by $output_ptr.
        //             (i32.store (get_local $output_ptr) (i32.const 42))
        //         )
        //     )
        //     "#
        // ).expect("Failed to compile WAT to WASM");
        let wasm_binary = wabt::wat2wasm(r#"
        (module
            (import "env" "memory" (memory 1))
            (func $exec (export "exec")
                (param $input_ptr i32) (param $input_len i32) (param $output_ptr i32)
                ;; Assuming serialization of [1.0, 2.0, 3.0] f32 values into bytes is done externally
                ;; and we're directly storing these bytes into memory for demonstration.
                
                ;; Example byte representation of 1.0 as f32 in little-endian
                (i32.store8 (get_local $output_ptr) (i32.const 0x00))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 1)) (i32.const 0x00))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 2)) (i32.const 0x80))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 3)) (i32.const 0x3F))
                
                ;; Increment output_ptr by 4 bytes for the next f32 value
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 4)) (i32.const 0x00))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 5)) (i32.const 0x00))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 6)) (i32.const 0x00))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 7)) (i32.const 0x40))
                
                ;; Increment output_ptr by 8 bytes for the next f32 value
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 8)) (i32.const 0x00))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 9)) (i32.const 0x00))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 10)) (i32.const 0x40))
                (i32.store8 (i32.add (get_local $output_ptr) (i32.const 11)) (i32.const 0x40))
            )
        )
    "#).expect("Failed to compile WAT to WASM");
        // Prepare data and a buffer for the result.
        let input_data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Example input.
        let mut result_buffer: Vec<u8> = vec![0; 12]; // Buffer to hold the result.

        exec_wasm_with_data(&wasm_binary, &input_data, &mut result_buffer)
            .expect("WASM execution failed");

        println!("result buffer {:?}", &result_buffer);
        // Verify that the result buffer contains the expected value
        assert_eq!(&result_buffer, &[0, 0, 128, 63, 0, 0, 0, 64, 0, 0, 64, 64], "Unexpected result in the buffer");
        // let res = exec_wasm(&binary).unwrap();
        // assert_eq!(res, Some(RuntimeValue::I32(1337)))
    }

    // #[test]
    // fn exec_wasm_with_data_works() {
    //     let binary = wabt::wat2wasm(
    //         r#"
    //         (module
    //         (type $t0 (func (param i32 i32) (result i32)))
    //         (import "env" "memory" (memory $env.memory 16))
    //         (func $exec (export "exec") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    //             (i32.load8_u
    //             (local.get $p0)))
    //         (global $g0 (mut i32) (i32.const 1048576))
    //         (global $__data_end (export "__data_end") i32 (i32.const 1048576))
    //         (global $__heap_base (export "__heap_base") i32 (i32.const 1048576)))
    //         "#,
    //     )
    //     .unwrap();

    //     let expected_result = 12u8;

    //     let data = [expected_result];

    //     let res = exec_wasm_with_data(&binary, &data).unwrap();
    //     assert_eq!(res, Some(RuntimeValue::I32(expected_result as i32)))
    // }
}
