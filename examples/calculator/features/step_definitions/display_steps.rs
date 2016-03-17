use cucumber::CucumberRegistrar;
use cucumber::InvokeResponse;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {

  Then!(c; "^the display says (\\d+)", |_, world: &mut CalculatorWorld, (number,): (i32,)| {
    InvokeResponse::check_eq(
      world.calculator.display_contents(),
      number
    )
  });

}
