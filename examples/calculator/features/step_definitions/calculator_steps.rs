use cucumber::CucumberRegistrar;

use support::env::CalculatorWorld;
use calculator::CalculatorCommand;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {

  Given!(c,
         "^a clear calculator$",
         |_, world: &mut CalculatorWorld, ()| {
           world.calculator.clear();
         });

  When!(c, "^I begin adding$", |_, world: &mut CalculatorWorld, ()| {
    world.last_response = Some(world.calculator.push_command(CalculatorCommand::Add));
  });

  When!(c,
        "^I begin subtracting$",
        |_, world: &mut CalculatorWorld, ()| {
          world.last_response = Some(world.calculator.push_command(CalculatorCommand::Minus));
        });

  When!(c,
        "^I input (-)?(\\d+)$",
        |_, world: &mut CalculatorWorld, (negate, mut val): (bool, i32)| {
          if negate {
            val = -val
          }
          world.last_response = Some(world.calculator.push_command(CalculatorCommand::Number(val)));
        });
}
