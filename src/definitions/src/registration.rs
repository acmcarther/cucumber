//! Logic for registering step definitions

use state::{Cucumber, Step};
use external_regex::Regex;

/// An interface for registering steps
///
/// This is a rough interface, because it requires specifying file and line information manually. Prefer using [the macros](../../index.html#macros).
///
/// See [WorldRunner](../../runner/struct.WorldRunner.html) for the primary implementer.
pub trait CucumberRegistrar<World> {
  fn given(&mut self, file: &str, line: u32, Regex, Step<World>);
  fn when(&mut self, file: &str, line: u32, Regex, Step<World>);
  fn then(&mut self, file: &str, line: u32, Regex, Step<World>);
}

impl <World> CucumberRegistrar<World> for Cucumber<World> {
  fn given(&mut self, file: &str, line: u32, regex: Regex, step: Step<World>) {
    self.insert_step(format!("{}:{}", file, line), regex, step)
  }

  fn when(&mut self, file: &str, line: u32, regex: Regex, step: Step<World>) {
    self.insert_step(format!("{}:{}", file, line), regex, step)
  }

  fn then(&mut self, file: &str, line: u32, regex: Regex, step: Step<World>) {
    self.insert_step(format!("{}:{}", file, line), regex, step)
  }
}

#[cfg(test)]
mod test {
  // TODO
}
