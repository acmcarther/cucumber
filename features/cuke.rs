#[macro_use]
extern crate cucumber;

mod step_definitions;
mod support;

use support::env::CucumberWorld;

use step_definitions::{
  registration_steps,
  matcher_steps
};

#[test]
fn main() {
  cucumber::start(
    CucumberWorld::new(),
    &[
      &registration_steps::register_steps,
      &matcher_steps::register_steps,
    ]
  );
}
