pub fn bubble_sort<T: Ord>(array: &mut Vec<T>) {
	let mut sorted = false;
	let mut length = array.len();

	while !sorted {
		sorted = true;
		for i in 0..length - 1 {
			if array[i] > array[i + 1] {
				array.swap(i, i + 1);
				sorted = false;
			}
		}
		length -= 1
	}
}

#[cfg(test)]
mod tests {
	use super::bubble_sort;
	#[test]
	fn test_bubble_sort() {
		let mut array = vec![9, 8, 7, 6];
		bubble_sort(&mut array);
		assert_eq!(array, vec![6, 7, 8, 9]);
	}
}
