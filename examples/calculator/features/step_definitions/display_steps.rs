use cucumber::{
  CucumberRegistrar,
  InvokeResponse
};
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {
  Then!(c, "^the (?:next )?result is (-)?(\\d+)$", |_, world: &mut CalculatorWorld, (negate, mut val): (bool, i32)| {
    if negate { val = -val }
    InvokeResponse::check_eq(world.calculator.evaluate(), val)
  });

  Then!(c, "^the last message includes \"(.*)\"$", |_, world: &mut CalculatorWorld, (message,): (String,)| {
    match world.last_response {
      None => InvokeResponse::fail_from_str("No last message"),
      Some(ref msg) => InvokeResponse::check(msg.to_string().contains(&message))
    }
  });
}
