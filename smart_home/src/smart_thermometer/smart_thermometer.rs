use std::fmt::{Display, Formatter, Result};

pub struct SmartThermometer {
	description: String,
	/// Temperature in Celsius (°C)
	last_temperature_mesurement: f64,
}

impl SmartThermometer {
	// cinstructor
	pub fn new(description: String) -> Self {
		Self {
			description,
			last_temperature_mesurement: 0.0,
		}
	}

	pub fn get_description(&self) -> &String {
		&self.description
	}

	pub fn get_temperature(&self) -> f64 {
		self.last_temperature_mesurement
	}

	pub fn set_temperature(&mut self, temperature: f64) {
		self.last_temperature_mesurement = temperature;
	}
}

impl Display for SmartThermometer {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"Thermometer: {}\n    temperature: {:.1}°C",
			self.get_description(),
			self.get_temperature(),
		)
	}
}
