use state::Cucumber;
use external_regex::Regex;
use definitions::Step;

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

// NOTE: These are capitalized to follow Cucumber general conventions, rather than Rust
#[macro_export]
macro_rules! Given {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.given(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

#[macro_export]
macro_rules! When {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.when(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

#[macro_export]
macro_rules! Then {
  ($cuke:ident; $regex:expr, $body:expr) => {{
    use $crate::regex;
    $cuke.then(file!(), line!(), regex::build($regex), Box::new(move |cuke, world, args| {
      ($body)(cuke, world, try_destructure!(args))
    }))
  }}
}

#[cfg(test)]
mod test {
  use super::*;
  use state::Cucumber;
  use response::{Step,  InvokeResponse, StepArg};
  use regex;

  // TODO: Tests for the macros. Exercise:
  //   - No args
  //   - One Arg
  //   - Several Args
  //   - Mismatched arg count
  //   - Mistyped args

  #[test]
  fn cuke_add_step() {
    type World = u32;
    let mut cuke: Cucumber<World> = Cucumber::new();
    cuke.given(file!(), line!(), regex::build("^I do a basic thing$"), Box::new(move |_, _, _| InvokeResponse::Success));
  }

  #[test]
  fn cuke_find_match() {
    type World = u32;
    let mut cuke: Cucumber<World> = Cucumber::new();
    cuke.given("file", 0, regex::build("^I do (\\d+) basic things?$"), Box::new(move |_, _, _| InvokeResponse::Success));

    let mut matches = cuke.find_match("I do 6 basic things");
    assert!(matches.len() == 1);
    let first_match = matches.pop().unwrap();
    assert_eq!(first_match, Step {id: "0".to_owned(), source: "file:0".to_owned(), args: vec!(StepArg { pos: Some(5), val: Some("6".to_owned())}) });
  }
}
