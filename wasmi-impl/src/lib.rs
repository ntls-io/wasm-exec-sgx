#![no_std]
use wasmi::{
    self, memory_units::Pages, ExternVal, ImportsBuilder, MemoryInstance, ModuleInstance,
    NopExternals, RuntimeValue,
};

static ENTRYPOINT: &str = "exec";

#[derive(Debug)]
pub enum ExecWasmError {
    WasmiError(wasmi::Error),
}

impl From<wasmi::Error> for ExecWasmError {
    fn from(err: wasmi::Error) -> Self {
        Self::WasmiError(err)
    }
}

pub fn exec_wasm_with_data(
    binary: &[u8],
    data: &[u8],
) -> Result<Option<RuntimeValue>, ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;

    // Calculate the memory size always be larger than `data`
    let data_size = &data.len();
    let num_pages = match data_size % (64 * 1024) {
        0 => data_size / (64 * 1024),
        _ => data_size / (64 * 1024) + 1,
    };
    let mem_instance = MemoryInstance::alloc(Pages(num_pages), Pages(num_pages))?;

    // TODO: Error Handling
    mem_instance.set(0, data).unwrap();

    let imports = [ExternVal::Memory(mem_instance)];

    // TODO: This panics. We probably want to run start functions
    let instance = ModuleInstance::with_externvals(&module, imports.iter())?.assert_no_start();

    Ok(instance.invoke_export(
        ENTRYPOINT,
        &[RuntimeValue::I32(0), RuntimeValue::I32(data.len() as i32)],
        &mut NopExternals,
    )?)
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
