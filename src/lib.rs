#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate regex as external_regex;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate itertools;

mod server;

pub mod definitions;
pub mod destructuring;
pub mod request;
pub mod response;
pub mod runner;
pub mod regex;
pub mod state;

pub use runner::WorldRunner;
pub use server::Server;

use std::process::{Command, Stdio};

pub fn ruby_command() -> Command {
  let mut command = Command::new("cucumber");
  command.stdout(Stdio::inherit());
  command.stderr(Stdio::inherit());
  command
}
