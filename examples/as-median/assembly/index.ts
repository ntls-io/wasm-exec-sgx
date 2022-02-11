
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
export function exec(msg: usize, msg_len: usize, out_ptr: usize, out_size: usize): i32 {
	/* error if msg size is not a multiple of 32 in bits */
	if (msg_len % 4 != 0) { return -1 }
	let count = msg_len / 4;
	let val = new Array<i32>(count as i32);

	/* convert length to bits from octets */
	let len = (msg_len * 8) as i32;

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

	/* create and populate the output buffer */
	out_size = 64 / 8;
	out_ptr = memory.data(64);
	store<f64>(out_ptr, median)

	return 0;
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
