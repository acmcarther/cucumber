use cucumber::state::Cucumber;

pub struct CucumberWorld {
  pub cucumber: Cucumber<u32>
}

impl CucumberWorld {
  pub fn new() -> CucumberWorld {
    CucumberWorld { cucumber: Cucumber::new() }
  }
}
