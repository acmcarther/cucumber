extern crate calculator;

#[macro_use]
extern crate cucumber;

mod step_definitions;
mod support;

use support::env::CalculatorWorld;
use step_definitions::{
  calculator_steps,
  display_steps
};

#[test]
fn main() {
  cucumber::start(
    CalculatorWorld::new(),
    &[
      &calculator_steps::register_steps,
      &display_steps::register_steps,
    ]
  );
}
