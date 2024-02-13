// #![no_std]
#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
use serde_json::{Value, json};
extern crate alloc;

use core::slice;
use core::mem;

/// Calculates the average of a slice of f32 numbers and writes the result to a provided memory location.
///
/// # Parameters
///
/// - `input_ptr`: A pointer to the start of the input data (f32 numbers encoded as bytes).
/// - `input_len`: The number of bytes in the input data.
/// - `output_ptr`: A pointer to where the result (f32 number encoded as bytes) should be stored.
///
/// # Safety
///
/// This function is unsafe because it does not perform bounds checking on the input or output pointers.
/// The caller must ensure that the provided memory is valid and that the input data correctly represents
/// a sequence of f32 numbers.
#[no_mangle]
pub unsafe extern "C" fn exec(input_ptr: *const u8, input_len: usize, output_ptr: *mut u8) {
    // Calculate the number of f32 elements in the input.
    let num_elements = input_len / mem::size_of::<f32>();

    // Convert the input bytes to a slice of f32 numbers.
    let input_slice = unsafe {slice::from_raw_parts(input_ptr as *const f32, num_elements)};

    // Calculate the average.
    let sum: f32 = input_slice.iter().sum();
    let average = if num_elements > 0 {
        sum / num_elements as f32
    } else {
        0.0
    };

    let result_message = format!("this is the result of the computation  {:.1}", average);

     let message_bytes = result_message.as_bytes();
    // Write the result to the provided memory location.
    // let output_slice = unsafe {slice::from_raw_parts_mut(output_ptr as *mut f32, 1) };
    // output_slice[0] = average;
    let output_bytes = unsafe { slice::from_raw_parts_mut(output_ptr, message_bytes.len())};
    output_bytes.copy_from_slice(message_bytes);
}
