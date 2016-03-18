use cucumber::definitions::registration::CucumberRegistrar;
use cucumber::response::InvokeResponse;
use support::env::CucumberWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CucumberWorld>) {
  Then!(c; "^the current tag state contains the tag \"(.*)\"$", |_, _, _| {
    InvokeResponse::pending("TODO")
  });
}

