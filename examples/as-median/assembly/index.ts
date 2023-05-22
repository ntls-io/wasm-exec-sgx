
import { JSONEncoder } from "assemblyscript-json"
import { parse, Arr, Integer, Value } from "assemblyscript-json/JSON";

export function exec(msg: usize, msg_size: usize, out_ptr: usize, out_size: usize): i32 {

	/* load input buffer into array */
	let in_buf = new Uint8Array(msg_size as i32);
	for(let i: usize = 0; i < msg_size; i++) {
	    let byte = load<u8>(msg + i);
	    in_buf[i as i32] = byte;
	}

	/* parse input buffer into an Array<i32> */
	let arr = parse(in_buf) as Arr;
	let value_array = arr.valueOf() as Array<Value>;
	let len = value_array.length;
	let sample = new Array<i32>(len);
	for(let i = 0; i < len; i++) {
		let entry = value_array[i] as Integer;
		sample[i] = entry.valueOf() as i32;
	}

	let median = calc_median(len, sample);

	/* encode JSON object */
	let encoder = new JSONEncoder();
	encoder.pushObject(null);
	encoder.setFloat("median", median);
	encoder.popObject();

	/* get pointer to buffer address */
	let out_buf = encoder.serialize().buffer;
	let out_buf_len = out_buf.byteLength;
	let out_buf_ptr = memory.data(8);
	store<ArrayBuffer>(out_buf_ptr, out_buf);
	store<usize>(out_ptr, out_buf_ptr);

	/* get pointer to buffer's size address */
	let out_size_ptr = memory.data(32);
	store<i32>(out_size_ptr, out_buf_len);
	store<usize>(out_size, out_size_ptr);

	return 0
}

/* recall that a median can have a fractional part */
function calc_median(len: usize, val: Array<i32>): f64 {
	val.sort();

	/*
	 * In the following, `lmid` and `rmid` are the same whenever `val` has
	 * odd length. Note also that the average of 2 identical values is
	 * simply the value itself.
	 */
	let lmid = calc_mid(len, 0) as i32;
	let rmid = calc_mid(len, 1) as i32;


	/* calculate aforementioned average by leveraging distributivity */
	let lhalf = (val[lmid] as f64) / 2.0;
	let rhalf = (val[rmid] as f64) / 2.0;
	let median = lhalf + rhalf;

	return median
}

function calc_mid(len: usize, rhs: bool): usize {
	/*
	 * Compute the index of the order statistic corresponding to the median
	 * if the former exists.  Otherwise, return a choice of one of the two
	 * closest order statistics.
	 */
	if (len % 2 == 0) {
		if (rhs) {
			let rmid = len / 2 + 1;
			return rmid;
		} else {
			let lmid = len / 2;
			return lmid;
		}
	} else {
		let mid = (len + 1) / 2;
		return mid;
	}
}
