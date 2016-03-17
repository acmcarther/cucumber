use cucumber::CucumberRegistrar;
use cucumber::InvokeResponse;
use support::env::CalculatorWorld;
use std::str::FromStr;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {

  When!(c; "^the calculator is cleared$", |_, ref mut world, _| {
    world.calculator.clear();
    InvokeResponse::Success
  });

  Given!(c; "^the calculator is clear$", |_, ref mut world, _| {
    world.calculator.clear();
    InvokeResponse::Success
  });

  When!(c; "^the number (\\d+) is entered$", |_, ref mut world, captures| {
    let (str,): (String,) = try_destructure!(captures);

    let capture = u32::from_str(&str).unwrap();
    world.calculator.enter(capture);
    InvokeResponse::Success
  });

}
