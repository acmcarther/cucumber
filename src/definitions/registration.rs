//! Logic for registering step definitions

use state::Cucumber;
use regex::Regex;
use event::request::InvokeArgument;

/// A "simpler" api-level step. Panic to fail.
pub type SimpleStep<World> = Box<Fn(&Cucumber<World>, &mut World, Vec<InvokeArgument>) + Send>;

/// An interface for registering steps
///
/// This is a rough interface, because it requires specifying file and line
/// information manually. Prefer using [the macros](../../index.html#macros).
///
/// See [WorldRunner](../../runner/struct.WorldRunner.html) for the primary
/// implementer.
pub trait CucumberRegistrar<World> {
  fn given(&mut self, file: &str, line: u32, Regex, SimpleStep<World>);
  fn when(&mut self, file: &str, line: u32, Regex, SimpleStep<World>);
  fn then(&mut self, file: &str, line: u32, Regex, SimpleStep<World>);
}

impl<World> CucumberRegistrar<World> for Cucumber<World> {
  fn given(&mut self, file: &str, line: u32, regex: Regex, step: SimpleStep<World>) {
    self.insert_step(format!("{}:{}", file, line), regex, step)
  }

  fn when(&mut self, file: &str, line: u32, regex: Regex, step: SimpleStep<World>) {
    self.insert_step(format!("{}:{}", file, line), regex, step)
  }

  fn then(&mut self, file: &str, line: u32, regex: Regex, step: SimpleStep<World>) {
    self.insert_step(format!("{}:{}", file, line), regex, step)
  }
}

#[cfg(test)]
mod test {
  // TODO
}
