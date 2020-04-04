//! Sstop is a simple utility built on top of libnotify
//! meant to notify about low battery status.
//!
//! Usage:
//! ```
//! sstop <min>
//! ```
//! run this at startup. It is not a daemon so it'll need to
//! be croned or something.
//!
//! Dependencies are: `acpi` and `libnotify`.
//! Works well with `dunst`

use anyhow::{Error, Result};
use notify_rust::{Notification, Timeout};
use std::process::Command;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

/// command line arguments representation
#[derive(StructOpt, Debug)]
struct Args {
	min: usize,
}

/// structure representing acpi output
#[derive(Debug)]
struct Acpi {
	batteries: Vec<Battery>,
}

impl Acpi {
	/// get acpi output and parse into self
	fn get() -> Result<Self> {
		let acpi = Command::new("acpi").output()?.stdout;
		let acpi = String::from_utf8(acpi)?;
		acpi.parse()
	}
}

impl FromStr for Acpi {
	type Err = Error;

	/// parsing
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let batteries: Result<Vec<Battery>> = s.lines().map(|x| x.parse::<Battery>()).collect();

		Ok(Acpi {
			batteries: batteries?,
		})
	}
}

/// structure representing a battery
#[derive(Debug)]
struct Battery {
	id: usize,
	charging: bool,
	percentage: usize,
}

impl Battery {
	/// access to libnotify and sending the notification
	fn notify(&self) -> Notification {
		Notification::new()
			.summary(&format!("BATTERY {} LOW: {}%", self.id, self.percentage))
			.timeout(Timeout::Never)
			.finalize()
	}

	/// checks if the battery is low
	fn is_low(&self, min: usize) -> bool {
		self.percentage < min && !self.charging
	}
}

impl FromStr for Battery {
	type Err = Error;

	/// acpi row parsing
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
				// could be minimized probably
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
