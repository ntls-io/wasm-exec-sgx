
export function exec(msg: usize, msg_len: usize, out_ptr: usize): i32 {
	/* error if msg size is not a multiple of 32 in bits */
	if (msg_len % 4 != 0) { return -1 }
	let count = msg_len / 4;
	let val = new Array<i32>(count as i32);

	/* convert length to bits from octets */
	let len = msg_len * 8;

	for(let i = 0; i < (len as i32); i++) {
		val[i] = load<i32>(msg + (i * 32));
	}
	val.sort();

	let median = calc_median(len, val);

	/* assume `out_ptr` points to a 64 bit memory buffer and populate it */
	store<f64>(out_ptr, median);

	return 0
}

/* recall that a median can have a fractional part */
function calc_median(len: usize, val: Array<i32>): f64 {
	/*
	 * In the following, `lmid` and `rmid` are the same whenever `val` has
	 * odd length. Note also that the average of 2 identical values is
	 * simply the value itself.
	 */
	let lmid = calc_mid(len, 0);
	let rmid = calc_mid(len, 1);


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
