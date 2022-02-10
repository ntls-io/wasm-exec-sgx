
export function alloc(count: i32): usize {
	/* allocate memory buffer */
	let buf = new Array<i32>(count);
	let buf_ptr = memory.data(32);

	/*
	 * dereference a pointer and mutate
	 * its underlying value
	 */
	store<Array<i32>>(buf_ptr, buf)

	return buf_ptr
}

// note that a median can have a fractional part
export function exec(msg: usize, msg_len: usize): f64 {
	let val = new Array<i32>(msg_len as i32);
	let len = msg_len as i32;
	for(let i = 0; i < len; i++) {
		val[i] = load<i32>(msg + (i * 32));
	}
	val.sort();

	/*
	 * in the following, `lmid` and `rmid` are the same
	 * whenever `val` has odd length. Note also that the
	 * average of 2 identical values is simply that value.
	 */
	let lmid = calc_mid(msg_len, 0);
	let rmid = calc_mid(msg_len, 1);

	let sum = (val[lmid as i32] + val[rmid as i32]) as f64;
	let median = sum / 2.0;
	return median;

}

function calc_mid(len: usize, rhs: bool): usize {
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
