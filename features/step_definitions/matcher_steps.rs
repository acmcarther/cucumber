use cucumber::definitions::registration::CucumberRegistrar;
use cucumber::event::response::InvokeResponse;
use cucumber::state::Cucumber;
use support::env::CucumberWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CucumberWorld>) {
  Given!(c; "^a new cucumber instance$", |_, world: &mut CucumberWorld, _| {
    world.cucumber = Cucumber::new();
    InvokeResponse::Success
  });
}
