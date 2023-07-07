use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
pub enum SwitchStatusEnum {
	On,
	Off,
}

impl Display for SwitchStatusEnum {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			SwitchStatusEnum::On => write!(f, "On"),
			SwitchStatusEnum::Off => write!(f, "Off"),
		}
	}
}
