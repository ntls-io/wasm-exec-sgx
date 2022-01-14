#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the median of an array of integers passed in as a JSON array
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> i32 {
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };
    let val: Vec<i32> = serde_json_wasm::from_slice(x).unwrap();
    // TODO: Val needs to be sorted...
    let mid = val.len() / 2;
    val[mid]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = b"[1,2,3,4,5]";
        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 3);
    }
}
