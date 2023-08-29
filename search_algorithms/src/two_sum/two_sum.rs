use std::collections::HashSet;

pub fn two_sum<T: Ord>(array: Vec<i32>, x: i32) -> (Option<i32>, Option<i32>)  {
	let mut set: HashSet<i32> = HashSet::from_iter(array.iter().cloned());

	for i in array.iter() {
		let y = if i < &(array.len() as i32) {
			x - array[*i as usize]
		} else { 0 };

		if set.contains(&y) {
			return (Some(array[*i as usize]), Some(y));
		} else {
			set.insert(*i);
		}
	}

	(None, None)
}

#[cfg(test)]
mod tests {
	use super::two_sum;
	#[test]
	fn test_bubble_sort() {
		let mut array = vec![2, 3, 3, 7, 8, 9, 11, 15];
		let res = two_sum::<Vec<i32>>(array.clone(), 6);
		assert_eq!(res, (Some(3), Some(3)));
	}
}
