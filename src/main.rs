#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate failure;
extern crate notify_rust;
#[macro_use]
extern crate structopt;

use failure::Error;
use notify_rust::{Notification, NotificationUrgency};
use std::fs::read_to_string;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {}

fn main() -> Result<(), Error> {
  let _args = Opts::from_args();
  loop {
    let level = get_power_level()?;
    if level < 20 {
      send_notification(level);
    }
    thread::sleep(Duration::from_secs(60 * 4));
  }
}

fn get_power_level() -> Result<u16, Error> {
  let mut file = read_to_string("/sys/class/power_supply/BAT0/capacity")?;
  let len = file.len() - 1;
  file.truncate(len);
  Ok(file.parse::<u16>()?)
}

fn send_notification(num: u16) {
  Notification::new()
    .summary("Low Battery")
    .urgency(NotificationUrgency::Normal)
    .body(&format!("{}% of battery remaining", num))
    .show()
    .unwrap();
}
