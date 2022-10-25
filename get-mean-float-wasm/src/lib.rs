#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the mean of an array of floats passed in as a JSON array.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> f64 {
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };

    //TODO - Fix error handling
    let val: Vec<f64> = serde_json_core::from_slice(&x).unwrap().0;

    let sum: f64 = val.iter().sum();
    sum as f64 / val.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_float_works() {
        let data = b"[8.0, 6.0, 8.0, 3.0, 7.0, 1.0, 9.0]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 6.0);
    }

    #[test]
    fn mean_float_works_decimal() {
        let data = b"[5.0, 2.0, 3.0, 1.0, 7.0, 6.0, 4.0, 9.0]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 4.625);
    }
}
