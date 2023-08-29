mod two_sum;

mod generate_array;

use two_sum::two_sum;
use generate_array::generate_array;

fn main() {
	let res = two_sum::<Vec<i32>>(generate_array(10_000), 20);

	print!("\x1B[2J"); // clear screen
	print!("\x1B[H"); // move cursor to (0, 0)
	println!("pairs: {:?}", res);
}
