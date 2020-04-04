use anyhow::{Error, Result};
use notify_rust::{Notification, Timeout};
use std::process::Command;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
	min: usize,
}

#[derive(Debug)]
struct Acpi {
	batteries: Vec<Battery>,
}

impl Acpi {
	fn get() -> Result<Self> {
		let acpi = Command::new("acpi").output()?.stdout;
		let acpi = String::from_utf8(acpi)?;
		acpi.parse()
	}
}

impl FromStr for Acpi {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let batteries: Result<Vec<Battery>> = s.lines().map(|x| x.parse::<Battery>()).collect();

		Ok(Acpi {
			batteries: batteries?,
		})
	}
}

#[derive(Debug)]
struct Battery {
	id: usize,
	charging: bool,
	percentage: usize,
}

impl Battery {
	fn notify(&self) -> Notification {
		Notification::new()
			.summary(&format!("BATTERY {} LOW: {}%", self.id, self.percentage))
			.timeout(Timeout::Never)
			.finalize()
	}

	fn is_low(&self, min: usize) -> bool {
		self.percentage < min && !self.charging
	}
}

impl FromStr for Battery {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut acpi: Vec<String> = s
			.replace(",", "")
			.split_ascii_whitespace()
			.map(String::from)
			.collect();

		let id: usize = {
			acpi[1].pop();
			acpi[1].parse()?
		};

		let percentage: usize = {
			acpi[3].pop();
			acpi[3].parse()?
		};

		let charging: bool = if &acpi[2] == "Charging" { true } else { false };

		Ok(Battery {
			id,
			charging,
			percentage,
		})
	}
}

fn main() -> Result<()> {
	let args = Args::from_args();
	let mut notified: Vec<usize> = vec![];
	loop {
		sleep(Duration::from_secs(1));
		let acpi = Acpi::get()?;

		acpi.batteries
			.iter()
			.filter(|x| match (x.is_low(args.min), notified.contains(&x.id)) {
				(true, true) => false,
				(true, false) => {
					notified.push(x.id);
					true
				}
				(false, true) => {
					notified = notified
						.clone()
						.into_iter()
						.filter(|y| y != &x.id)
						.collect();
					false
				}
				(false, false) => false,
			})
			.map(|x| x.notify().show().unwrap())
			.count();
	}
}
