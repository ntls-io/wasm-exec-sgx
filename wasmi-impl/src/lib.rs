#![no_std]
use wasmi::{self, ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

#[derive(Debug)]
pub enum ExecWasmError {
    WasmiError(wasmi::Error),
}

impl From<wasmi::Error> for ExecWasmError {
    fn from(err: wasmi::Error) -> Self {
        Self::WasmiError(err)
    }
}

pub fn exec_wasm(binary: &[u8]) -> Result<Option<RuntimeValue>, ExecWasmError> {
    let module = wasmi::Module::from_buffer(binary)?;
    // TODO: This panics. We probably want to run start functions
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())?.assert_no_start();
    Ok(instance.invoke_export("test", &[], &mut NopExternals)?)
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
                (func (export "test") (result i32)
                    i32.const 1337
                )
            )
            "#,
        )
        .unwrap();
        let res = exec_wasm(&binary).unwrap();
        assert_eq!(res, Some(RuntimeValue::I32(1337)))
    }
}
