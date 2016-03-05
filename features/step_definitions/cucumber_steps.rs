use cucumber::CucumberRegistrar;
use cucumber::InvokeResponse;
use cucumber::helpers::r;
use support::env::CucumberWorld;

#[allow(dead_code)]
pub fn register_steps(cuke: &mut CucumberRegistrar<CucumberWorld>) {
  Then!(cuke, r("^the current tag state contains the tag \"(.*)\"$"), Box::new(move |_, _, _| {
    InvokeResponse::pending("TODO")
  }));
}

