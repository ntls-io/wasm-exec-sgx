#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the mean of an array of integers passed in as a JSON array. Return value will be rounded down.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> i32 {
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };
    let mut val: Vec<i32> = serde_json_wasm::from_slice(x).unwrap();

    let sum: i32 = val.iter().sum();
    sum as i32 / val.len() as i32

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_int_works() {
        let data = b"[8, 6, 8, 3, 7, 1, 9]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 6);
    }

    #[test]
    fn mean_int_works_round() {
        let data = b"[5, 2, 3, 1, 7, 6, 4, 9]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 4);
    }
}
