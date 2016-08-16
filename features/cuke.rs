#[macro_use]
extern crate cucumber;

extern crate tempdir;
extern crate itertools;

mod step_definitions;
mod support;

use support::env::CucumberWorld;

use step_definitions::project_steps;

fn cuke() {
  cucumber::start(CucumberWorld::new(), &[&project_steps::register_steps]);
}

fn main() {
  cuke()
}
