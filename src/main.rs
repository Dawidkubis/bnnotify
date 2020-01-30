extern crate notify_rust;
#[macro_use]
extern crate anyhow;

use notify_rust::Notification;
use notify_rust::Timeout;
use anyhow::{Result, Error};
use std::process::Command;
use std::str::FromStr;

struct Acpi {
	batteries: Vec<Battery>,
}

struct Battery {
	id: usize,
	status: String,
	percentage: usize,
	time: String,
}

impl FromStr for Battery {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		//TODO
	}
}

fn main() -> Result<()> {

	Notification::new()
    	.summary("Firefox News")
    	.body("This will almost look like a real firefox notification.")
    	.icon("firefox")
    	.timeout(Timeout::Milliseconds(6000)) //milliseconds
    	.show()?;

    Ok(())
}
