use cucumber::CucumberRegistrar;
use cucumber::InvokeResponse;
use cucumber::helpers::r;
use std::str::FromStr;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(cuke: &mut CucumberRegistrar<CalculatorWorld>) {

  Then!(cuke, r("^the display says (\\d+)"), Box::new(move |_, ref mut world, mut captures| {
    let displayed_value = world.calculator.display_contents();
    let str = cuke_pop_string!(captures);
    let capture = i32::from_str(&str).unwrap();

    if displayed_value == capture {
      InvokeResponse::Success
    } else {
      InvokeResponse::fail(format!("Displayed value [{}] did not equal expected value [{}]", displayed_value, capture))
    }
  }));

}
