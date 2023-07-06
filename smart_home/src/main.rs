mod smart_outlet;
mod smart_thermometer;
mod state;
mod switch_status_enum;

use smart_outlet::SmartOutlet;
use smart_thermometer::SmartThermometer;
use state::{State, ToggleState};
use switch_status_enum::SwitchStatusEnum;

use std::thread::sleep;
use std::time::Duration;

fn main() {
	let sleep_duration = Duration::from_millis(1000);
	// init outlet
	let mut outlet = SmartOutlet::new(String::from("Bed Room"));
	// init thermometer
	let mut thermometer = SmartThermometer::new(String::from("Outside"));

	let amperage_state = State::new(vec![1000.0, 2000.0]);
	let voltage_state = State::new(vec![4000.0, 5000.0]);
	let temperature_state = State::new(vec![23.0, 24.0]);
	let mocked_switch_state = State::new(vec![SwitchStatusEnum::On, SwitchStatusEnum::Off]);

	// main loop
	loop {
		let switch_state = mocked_switch_state.toggle();

		// change state
		let params: (f64, f64) = match switch_state {
			SwitchStatusEnum::On => (amperage_state.toggle(), voltage_state.toggle()),
			SwitchStatusEnum::Off => (0.0, 0.0),
		};

		let (voltage, amperage) = params;

		outlet.set_power_consumption(voltage, amperage);
		outlet.set_power_state(switch_state);

		thermometer.set_temperature(temperature_state.toggle());

		print!("\x1B[2J"); // clear screen
		print!("\x1B[H"); // move cursor to (0, 0)
		println!("{}", outlet);
		println!("{}", thermometer);

		sleep(sleep_duration);
	}
}
