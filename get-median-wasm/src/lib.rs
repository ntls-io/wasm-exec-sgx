#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the median of an array of floats passed in as a JSON array.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> f64 {
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };

    //TODO - Fix error handling
    let mut val: Vec<f64> = serde_json_core::from_slice(&x).unwrap().0;

    val.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let val_len = val.len();
    if val_len % 2 == 0 {
        let mid = val_len / 2;
        (val[mid - 1] + val[mid]) / 2.0
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
        let data = b"[8.0, 6.0, 3.0, 3.0, 7.0, 1.0, 9.0]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 6.0);
    }

    #[test]
    fn it_works_even() {
        let data = b"[5.0, 2.0, 3.0, 1.0, 7.0, 6.0, 4.0, 9.0]";

        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 4.5);
    }
}
