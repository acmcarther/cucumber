use cucumber::definitions::registration::CucumberRegistrar;
use cucumber::response::InvokeResponse;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {
  Then!(c; "^the display( doesn't)? says? (\\d+)", |_, world: &mut CalculatorWorld, (negate, number): (Option<String>, i32)| {
    if negate.is_none() {
      InvokeResponse::check_eq(world.calculator.display_contents(), number)
    } else {
      InvokeResponse::check_not_eq(world.calculator.display_contents(), number)
    }
  });
}
