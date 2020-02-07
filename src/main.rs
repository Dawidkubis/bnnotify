use anyhow::{Error, Result};
use lazy_static::lazy_static;
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

	fn notify(&self) -> Vec<Notification> {
		self.batteries.iter()
			.map(|x| x.notify())
			.filter(Option::is_some)
			.map(|x| {
				if let Some(s) = x {
					s
				} else {
					panic!("You have now entered a place of code I once thought is impossible to reach. Well done and good luck fixing this.")
				}
			})
			.collect()
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
	fn notify(&self) -> Option<Notification> {
		if self.percentage < ARGS.min && !self.charging {
			return Some(
				Notification::new()
					.summary(&format!("BATTERY {} LOW: {}%", self.id, self.percentage))
					.timeout(Timeout::Never)
					.finalize(),
			);
		}
		None
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

lazy_static! {
	static ref ARGS: Args = Args::from_args();
}

fn main() -> Result<()> {
	loop {
		sleep(Duration::from_secs(1));
		let acpi = dbg!(Acpi::get()?);

		acpi.notify().into_iter().map(|x| x.show().unwrap()).count();
	}
}
