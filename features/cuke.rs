#[macro_use]
extern crate cucumber;

mod step_definitions;
mod support;

use cucumber::{ WorldRunner, Server };
use cucumber::helpers::cucumber_command;

use support::env::CucumberWorld;
use step_definitions::cucumber_steps;

fn main() {
  let mut runner = WorldRunner::new(CucumberWorld::new());

  // Register all steps
  cucumber_steps::register_steps(&mut runner);

  let server = Server::new(runner);
  let mut listener = server.start(Some("0.0.0.0:7878"));

  let status = cucumber_command()
    .spawn()
    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
    .wait().unwrap();

  listener.wait();

  std::process::exit(status.code().unwrap())
}
