use cucumber::definitions::registration::CucumberRegistrar;
use cucumber::response::InvokeResponse;
use cucumber::state::Cucumber;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {

  When!(c; "^the calculator is cleared$", |cuke: &Cucumber<CalculatorWorld>, world: &mut CalculatorWorld, _| {
    cuke.invoke("the calculator is clear", world, None)
  });

  Given!(c; "^the calculator is clear$", |_, world: &mut CalculatorWorld, _| {
    world.calculator.clear();
    InvokeResponse::Success
  });

  When!(c; "^the number (\\d+) is entered$", |_, world: &mut CalculatorWorld, (number,): (u32,)| {
    world.calculator.enter(number);
    InvokeResponse::Success
  });
}
