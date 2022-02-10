
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

export function exec(msg: usize, msg_len: usize): i32 {
	let val = new Array<i32>(msg_len as i32);
	let len = msg_len as i32;
	for(let i = 0; i < len; i++) {
		val[i] = load<i32>(msg + (i * 32));
	}
	val.sort();

	let mid = calc_mid(msg_len);
	return val[mid as i32];
}

function calc_mid(len: usize): usize {
	if (len % 2 == 0) {
		let mid = len / 2 + 1;
		return mid;
	} else {
		let mid = (len + 1) / 2;
		return mid;
	}
}
