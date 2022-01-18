#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the median of an array of integers passed in as a JSON array. Return value will be rounded down.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> i32 {
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };
    let mut val: Vec<i32> = serde_json_wasm::from_slice(x).unwrap();

    val.sort_unstable();

    let val_len = val.len();
    if val_len % 2 == 0 {
        let mid = val_len / 2;
        (val[mid - 1] + val[mid]) / 2
    } else {
        let mid = (val_len + 1) / 2;
        val[mid - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_odd() {
        let data = b"[8, 6, 3, 3, 7, 1, 9]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 6);
    }

    #[test]
    fn it_works_even() {
        let data = b"[5, 2, 3, 1, 7, 6, 4, 9]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 4);
    }
}
