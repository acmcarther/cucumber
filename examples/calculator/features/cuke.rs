extern crate calculator;

#[macro_use]
extern crate cucumber;

mod step_definitions;
mod support;

use cucumber::{ WorldRunner, Server };

use support::env::CalculatorWorld;
use step_definitions::{
  calculator_steps,
  display_steps
};

#[test]
fn main() {
  let mut runner = WorldRunner::new(CalculatorWorld::new());

  // Register all steps
  calculator_steps::register_steps(&mut runner);
  display_steps::register_steps(&mut runner);

  let server = Server::new(runner);
  // NOTE: Unused stop_rx needs to be held, or it will drop and close the server
  let (handle, stop_rx) = server.start(Some("0.0.0.0:7878"));

  let status = cucumber::ruby_command()
    .spawn()
    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
    .wait().unwrap();

  handle.join().unwrap();

  std::process::exit(status.code().unwrap())
}
