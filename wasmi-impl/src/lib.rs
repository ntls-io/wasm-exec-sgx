#![no_std]
extern crate alloc;

use core::cmp;
use alloc::vec::{self, Vec};
use wasmi::{
    self, memory_units::Pages, ExternVal, ImportsBuilder, MemoryInstance,
    ModuleInstance, NopExternals, RuntimeValue,
};

static ENTRYPOINT: &str = "exec";
static MAX_MEMORY_KIB: usize = 128 * 1024;
static MAX_PAGES: usize = match MAX_MEMORY_KIB % 64 {
    0 => MAX_MEMORY_KIB / 64,
    _ => MAX_MEMORY_KIB / 64 + 1,
};
static MEM_SMALL_BYTES: usize = 128 / 8;
static PAGE_SIZE_BYTES: usize = 64 * 1024;

#[derive(Debug)]
pub enum ExecWasmError {
    WasmiError(wasmi::Error),
}

impl From<wasmi::Error> for ExecWasmError {
    fn from(err: wasmi::Error) -> Self {
        Self::WasmiError(err)
    }
}

struct Proc {
    out_buffer: Vec<u8>,
    return_code: Option<RuntimeValue>,
}

impl<'a> Proc {
    pub fn bytes(&'a self) -> impl 'a + Iterator {
        self.out_buffer.iter()
    }
}

pub fn exec_wasm_with_data<'a>(
    binary: &[u8],
    data: &[u8],
    out_size: usize, /* output size in bytes */
) -> Result<Proc, ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;

    let data_size = data.len();

    /*
     *  Calculate the memory size to always be larger than `data`.  Small
     *  values (a WASM numeric vector or smaller) should always fit.
     */
    let calc_rem = |num_bytes| cmp::max(num_bytes, MEM_SMALL_BYTES) % PAGE_SIZE_BYTES;
    let calc_pages = |num_bytes| cmp::max(num_bytes, MEM_SMALL_BYTES) / PAGE_SIZE_BYTES;
    let combined_size = data_size + out_size;
    let num_pages = cmp::min(
        MAX_PAGES,
        match calc_rem(combined_size) {
            0 => calc_pages(combined_size),
            _ => calc_pages(combined_size) + 1,
        },
    );

    let mem_instance = MemoryInstance::alloc(Pages(num_pages), Some(Pages(MAX_PAGES)))?;

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
    let proc = Proc {
        out_buffer: vec![0u8; out_size],
        return_code,
    };
    mem_instance.get_into(data_size as u32, proc.out_buffer.as_mut_slice());
    Ok(proc)
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
