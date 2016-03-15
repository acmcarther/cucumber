use cucumber::CucumberRegistrar;
use cucumber::InvokeResponse;
use cucumber::helpers::r;
use support::env::CalculatorWorld;
use std::str::FromStr;

#[allow(dead_code)]
pub fn register_steps(cuke: &mut CucumberRegistrar<CalculatorWorld>) {

  When!(cuke, r("^the calculator is cleared$"), Box::new(move |_, ref mut world, _| {
    world.calculator.clear();
    InvokeResponse::Success
  }));

  Given!(cuke, r("^the calculator is clear$"), Box::new(move |_, ref mut world, _| {
    world.calculator.clear();
    InvokeResponse::Success
  }));

  When!(cuke, r("^the number (\\d+) is entered$"), Box::new(move |_, ref mut world, captures| {
    let (str,): (String,) = try_destructure!(captures);

    let capture = u32::from_str(&str).unwrap();
    world.calculator.enter(capture);
    InvokeResponse::Success
  }));

}
