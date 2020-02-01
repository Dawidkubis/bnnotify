extern crate notify_rust;
#[macro_use]
extern crate anyhow;

use anyhow::{Error, Result};
use notify_rust::Notification;
use notify_rust::Timeout;
use std::process::Command;
use std::str::FromStr;

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
	status: String,
	percentage: usize,
	time: String,
}

impl FromStr for Battery {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut acpi: Vec<String> = s
			.split_ascii_whitespace()
			.map(|x| String::from(x))
			.collect();

		let id: usize = {
			acpi[1].pop();
			acpi[1].parse()?
		};

		let percentage: usize = {
			acpi[3].pop();
			acpi[3].pop();
			acpi[3].parse()?
		};

		Ok(Battery {
			id,
			status: acpi[2].clone(),
			percentage,
			time: acpi[4].clone(),
		})
	}
}

fn main() -> Result<()> {
	//Notification::new()
	//.summary("Firefox News")
	//.body("This will almost look like a real firefox notification.")
	//.icon("firefox")
	//.timeout(Timeout::Milliseconds(6000))
	//.show()?;

	println!("{:?}", Acpi::get()?);

	Ok(())
}
