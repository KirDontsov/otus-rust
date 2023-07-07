use rand::prelude::*;

pub fn generate_array(n: i32) -> Vec<i32> {
	let mut rng = rand::thread_rng();

	let mut nums: Vec<i32> = (1..n).collect();
	nums.shuffle(&mut rng);
	nums
}
