use cucumber::CucumberRegistrar;
use cucumber::InvokeResponse;
use std::str::FromStr;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {

  Then!(c; "^the display says (\\d+)", |_, ref mut world, captures| {
    let displayed_value = world.calculator.display_contents();
    let (str,): (String,) = try_destructure!(captures);
    let capture = i32::from_str(&str).unwrap();

    if displayed_value == capture {
      InvokeResponse::Success
    } else {
      InvokeResponse::fail(format!("Displayed value [{}] did not equal expected value [{}]", displayed_value, capture))
    }
  });

}
