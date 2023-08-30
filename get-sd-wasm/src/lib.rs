#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use core::slice;
use serde_json::Value;

/// Calculates the SD of an array of integers passed in as a JSON array. Returns a float.
///
/// # Safety
/// The caller needs to ensure that `msg` is a valid pointer, and points to a slice with `msg_len` items
#[no_mangle]
pub unsafe extern "C" fn exec(msg: *const u8, msg_len: u32) -> f32 {
    let x = unsafe { slice::from_raw_parts(msg, msg_len as usize) };
    let val: Vec<Value> = match serde_json::from_slice(&x) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error deserializing JSON: {}", err);
            return 0.0;
        }
    };

    // Convert integers and floats to f32
    let float_vals: Vec<f32> = val
        .iter()
        .filter_map(|v| match v {
            Value::Number(n) => Some(
                n.as_f64()
                    .expect("Failed to convert number to f64") as f32
            ),
            _ => None,
        })
        .collect();
    
    let len = float_vals.len();
    if len == 0 || len == 1 {
        return 0.0;
    }
    let sum: f32 = float_vals.iter().sum();
    let data_mean = sum / len as f32;

    let variance = float_vals.iter().map(|&value| {
        let diff = data_mean - value;
        diff * diff
    }).sum::<f32>() / (len as f32 - 1.0);

    variance.sqrt()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sd_works() {
        let data = b"[8, 6, 8, 3, 7, 1, 9]";
        let res = unsafe { exec(data.as_ptr(), data.len() as u32) };
        assert_eq!(res, 2.9439204);
    }

}
