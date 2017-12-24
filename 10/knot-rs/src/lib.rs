extern crate hex;

pub const LIST_SIZE: usize = 256;
pub const NUM_ROUNDS: usize = 64;
pub const BLOCK_SIZE: usize = 16;

pub fn reverse(arr: &mut [u32], start: usize, end: usize) {
	let mut start = start;
	let mut end = end;
	let len = arr.len();

	while start <= end {
		let tmp = arr[start % len];
		arr[start % len] = arr[end % len];
		arr[end % len] = tmp;

		start += 1;
		end -= 1;
	}
}

pub fn knot_hash(input: &str) -> String {
	let mut lengths: Vec<u32> = input.bytes()
		.map(|s| s as u32)
		.collect();

	// Magic numbers from instructions
	for length in [17, 31, 73, 47, 23].iter() {
		lengths.push(*length as u32);
	}

	let mut list: [u32; LIST_SIZE] = [0; LIST_SIZE];
	for i in 0..LIST_SIZE {
		list[i] = i as u32;
	}

	let mut skip = 0;
	let mut position = 0;
	for _ in 0..NUM_ROUNDS {
		for length in &lengths {
			reverse(&mut list, position as usize, (position + length - 1) as usize);

			position += length + skip;
			skip += 1;
		}
	}


	const DENSE_HASH_SIZE: usize = LIST_SIZE / BLOCK_SIZE;
	let mut dense_hash: [u8; DENSE_HASH_SIZE] = [0; DENSE_HASH_SIZE];
	for i in 0..DENSE_HASH_SIZE {
		for j in 0..BLOCK_SIZE {
			dense_hash[i] = dense_hash[i] ^ list[i * DENSE_HASH_SIZE + j] as u8;
		}
	}

	hex::encode(dense_hash)
}
