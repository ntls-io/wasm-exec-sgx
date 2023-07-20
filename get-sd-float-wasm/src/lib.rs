#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;

/// Calculates the SD of an array of floats passed in as a JSON array.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> f32 {
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };
    let val: Vec<f32> = serde_json_core::from_slice(&x).unwrap().0;
    
    let sum: f32 = val.iter().sum();
    let data_mean = sum as f32 / val.len() as f32;

    let variance = val.iter().map(|value| {
        let diff = data_mean - (*value as f32);
        diff * diff
    }).sum::<f32>() / (val.len() as f32 - 1.0);
    variance.sqrt()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sd_works() {
        let data = b"[9.0, 6.0, 3.0, 3.0, 6.0, 9.0]";
        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 2.6832814);
    }

}
