use rand::{prelude::SliceRandom, thread_rng};

pub fn generate_array(n: i32) -> Vec<i32> {
	let mut nums: Vec<i32> = (1..n).collect();
	nums.shuffle(&mut thread_rng());
	nums
}

#[cfg(test)]
mod tests {
	use super::generate_array;
	#[test]
	fn test_generate_array() {
		let array = generate_array(5);
		assert_eq!(array.len(), vec![1, 2, 3, 4].len());
	}
}
