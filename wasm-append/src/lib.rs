#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the mean of an array of floats passed in as a JSON array.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec_append(msg_1: *const u8, msg_len_1: u32, msg_2: *const u8, msg_len_2: u32) -> i32 {
    let x = unsafe { slice::from_raw_parts(msg_1, msg_len_1 as usize) };
    let y = unsafe { slice::from_raw_parts(msg_2, msg_len_2 as usize) };
    
    let mut val_1: Vec<i32> = serde_json_wasm::from_slice(x).unwrap();
    let mut val_2: Vec<i32> = serde_json_wasm::from_slice(y).unwrap();

    val_1.append(&mut val_2);

    let total = val_1.iter().sum();

    // Export val_1 to CosmosDB

    // Update this to return 1 (Bool) when successful
    return total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_append_works() {
        let data_1 = b"[1, 2, 3, 4, 5]";
        let data_2 = b"[6, 7, 8, 9, 10]";

        let res = unsafe { exec_append(data_1.as_ptr(), data_1.len() as u32, data_2.as_ptr(), data_2.len() as u32) };
        assert_eq!(res, 55);
    }

}
