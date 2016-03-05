use cucumber::WorldRunner;

pub struct CucumberWorld {
  pub runner: WorldRunner<u32>
}

impl CucumberWorld {
  pub fn new() -> CucumberWorld {
    CucumberWorld { runner: WorldRunner::new(0) }
  }
}
