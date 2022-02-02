#![no_std]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use wasmi::{
    self,
    memory_units::{Bytes, Pages, RoundUpTo},
    ExternVal, ImportsBuilder, MemoryInstance, ModuleInstance, NopExternals, RuntimeValue,
};

static ENTRYPOINT: &str = "exec";

const MAX_MEMORY_KIB: usize = 128 * 1024;
const MAX_PAGES: Pages = Pages(MAX_MEMORY_KIB);

#[derive(Debug)]
pub enum ExecWasmError {
    WasmiError(wasmi::Error),
    WasmiReturn(i32),
    InvalidModule,
}

impl From<wasmi::Error> for ExecWasmError {
    fn from(err: wasmi::Error) -> Self {
        Self::WasmiError(err)
    }
}

pub struct WasmOutput {
    out_buffer: Vec<u8>,
    return_code: Option<RuntimeValue>,
}

impl WasmOutput {
    pub fn out_buffer(&self) -> &[u8] {
        self.out_buffer.as_slice()
    }
    pub fn return_code(&self) -> Option<RuntimeValue> {
        self.return_code
    }
}

pub fn calc_num_pages(data_size: usize, out_size: usize) -> Result<Pages, ExecWasmError> {
    let combined_size = match data_size.checked_add(out_size) {
        None => Err(ExecWasmError::InvalidModule),
        Some(size) => Ok(Bytes(size)),
    }?;

    let num_pages: Pages = combined_size.round_up_to();
    if num_pages > MAX_PAGES {
        Err(ExecWasmError::InvalidModule)
    } else {
        Ok(num_pages)
    }
}

pub fn exec_wasm_with_data(
    binary: &[u8],
    data: &[u8],
    out_size: usize,
) -> Result<Vec<u8>, ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;

    let data_size = data.len();

    let num_pages = calc_num_pages(data_size, out_size)?;
    let mem_instance = MemoryInstance::alloc(num_pages, Some(MAX_PAGES))?;

    // TODO: Error Handling
    mem_instance.set(0, data).unwrap();

    let imports = [ExternVal::Memory(mem_instance.clone())];

    // TODO: This panics. We probably want to run start functions
    let instance = ModuleInstance::with_externvals(&module, imports.iter())?.assert_no_start();

    let return_code = instance.invoke_export(
        ENTRYPOINT,
        &[RuntimeValue::I32(0), RuntimeValue::I32(data.len() as i32)],
        &mut NopExternals,
    )?;
    match return_code {
        Some(RuntimeValue::I32(0)) => {
            let mut wasm_output = vec![0u8; out_size];
            mem_instance.get_into(data_size as u32, wasm_output.as_mut_slice())?;
            Ok(wasm_output)
        }
        Some(RuntimeValue::I32(code)) => Err(ExecWasmError::WasmiReturn(code)),
        _ => Err(ExecWasmError::InvalidModule),
    }
}

pub fn exec_wasm(binary: &[u8]) -> Result<Option<RuntimeValue>, ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;
    // TODO: This panics. We probably want to run start functions
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())?.assert_no_start();
    Ok(instance.invoke_export(ENTRYPOINT, &[], &mut NopExternals)?)
}

#[cfg(test)]
mod tests {

    use super::*;
    use wabt;

    #[test]
    fn exec_wasm_works() {
        let binary = wabt::wat2wasm(
            r#"
            (module
                (func (export "exec") (result i32)
                    i32.const 1337
                )
            )
            "#,
        )
        .unwrap();
        let res = exec_wasm(&binary).unwrap();
        assert_eq!(res, Some(RuntimeValue::I32(1337)))
    }

    #[test]
    fn exec_wasm_with_data_works() {
        let binary = wabt::wat2wasm(
            r#"
            (module
            (type $t0 (func (param i32 i32) (result i32)))
            (import "env" "memory" (memory $env.memory 16))
            (func $exec (export "exec") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
                (i32.load8_u
                (local.get $p0)))
            (global $g0 (mut i32) (i32.const 1048576))
            (global $__data_end (export "__data_end") i32 (i32.const 1048576))
            (global $__heap_base (export "__heap_base") i32 (i32.const 1048576)))
            "#,
        )
        .unwrap();

        let expected_result = 12u8;

        let data = [expected_result];

        let res = exec_wasm_with_data(&binary, &data).unwrap();
        assert_eq!(res, Some(RuntimeValue::I32(expected_result as i32)))
    }
}
