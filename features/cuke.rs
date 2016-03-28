#[macro_use]
extern crate cucumber;

extern crate tempdir;

mod step_definitions;
mod support;

use support::env::CucumberWorld;

use step_definitions::{
  project_steps
};

#[test]
fn cuke() {
  cucumber::start(
    CucumberWorld::new(),
    &[
      &project_steps::register_steps,
    ]
  );
}

fn main() {
  cuke()
}
