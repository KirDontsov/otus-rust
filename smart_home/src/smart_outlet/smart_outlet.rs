use std::fmt::{Display, Formatter, Result};

use crate::switch_status_enum::SwitchStatusEnum;

pub struct SmartOutlet {
	description: String,
	power_state: SwitchStatusEnum,
	/// Power units (Watt)
	last_power_consumption_mesurement: f64,
}

impl SmartOutlet {
	// cinstructor
	pub fn new(description: String) -> Self {
		Self {
			description,
			power_state: SwitchStatusEnum::Off,
			last_power_consumption_mesurement: 0.0,
		}
	}

	pub fn set_power_state(&mut self, state: SwitchStatusEnum) {
		self.power_state = state;
	}

	pub fn get_power_state(&self) -> SwitchStatusEnum {
		self.power_state
	}

	pub fn get_description(&self) -> &String {
		&self.description
	}

	pub fn get_power_units(&self) -> f64 {
		self.last_power_consumption_mesurement
	}

	pub fn set_power_consumption(&mut self, volts: f64, ampers: f64) {
		self.last_power_consumption_mesurement = volts * ampers;
	}
}

impl Display for SmartOutlet {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"Outlet: {}\n    power: {}\n    consumptin: {:.1}kW",
			self.get_description(),
			self.get_power_state(),
			self.get_power_units() * 0.001,
		)
	}
}
