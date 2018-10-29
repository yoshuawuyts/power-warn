#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate failure;
extern crate notify_rust;
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
  let _ = Opts::from_args();
  loop {
    if is_using_battery()? {
      let level = get_power_level()?;
      if level < 5 {
        send_notification(level, NotificationUrgency::Critical);
      } else if level < 20 {
        send_notification(level, NotificationUrgency::Normal);
      }
    }
    thread::sleep(Duration::from_secs(60 * 8));
  }
}

fn is_using_battery() -> Result<bool, Error> {
  let file = read_to_string("/sys/class/power_supply/BAT0/status")?;
  Ok(file == "Discharging\n")
}

fn get_power_level() -> Result<u16, Error> {
  let mut file = read_to_string("/sys/class/power_supply/BAT0/capacity")?;
  let len = file.len() - 1;
  file.truncate(len);
  Ok(file.parse::<u16>()?)
}

fn send_notification(num: u16, level: NotificationUrgency) {
  Notification::new()
    .summary("Low Battery")
    .urgency(level)
    .body(&format!("{}% of battery remaining", num))
    .show()
    .unwrap();
}
