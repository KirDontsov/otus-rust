use std::cell::Cell;

use super::traits::ToggleState;

pub struct State<T> {
	payload: Vec<T>,
	index: Cell<usize>,
}

impl<T> State<T> {
	// cinstructor
	pub fn new(payload: Vec<T>) -> Self {
		assert!(!payload.is_empty());
		Self {
			payload,
			index: Cell::new(0),
		}
	}

	fn increment(&self) {
		let index = self.index.get() + 1;
		if index >= self.payload.len() {
			self.index.set(0);
		} else {
			self.index.set(index);
		}
	}
}

impl<T> ToggleState<T> for State<T>
where
	T: Copy,
{
	fn toggle(&self) -> T {
		let index = self.index.get();
		self.increment();
		self.payload[index]
	}
}
