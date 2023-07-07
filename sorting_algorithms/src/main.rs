mod bubble_sort;
mod generate_array;
mod quick_sort;

use std::time::Instant;

use bubble_sort::bubble_sort;
use generate_array::generate_array;
use quick_sort::quick_sort;

fn main() {
	let mut bubble_sort_arr: Vec<i32> = generate_array(10_000);
	let mut quick_sort_arr: Vec<i32> = generate_array(10_000);

	// benchmarks
	loop {
		// bubble_sort
		let bubble_sort_start = Instant::now();
		bubble_sort(&mut bubble_sort_arr);
		let bubble_sort_elapsed = bubble_sort_start.elapsed().as_millis();

		// quick_sort
		let quick_sort_start = Instant::now();
		quick_sort(&mut quick_sort_arr);
		let quick_sort_elapsed = quick_sort_start.elapsed().as_millis();

		print!("\x1B[2J"); // clear screen
		print!("\x1B[H"); // move cursor to (0, 0)
		println!("bubble_sort time: {} ms", bubble_sort_elapsed);
		println!("quick_sort time: {} ms", quick_sort_elapsed);
	}
}
