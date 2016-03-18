#[macro_use]
extern crate cucumber;

mod step_definitions;
mod support;

use cucumber::{ WorldRunner, Server };

use support::env::CucumberWorld;
use step_definitions::cucumber_steps;

#[test]
fn main() {
  let mut runner = WorldRunner::new(CucumberWorld::new());

  // Register all steps
  cucumber_steps::register_steps(&mut runner);

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
