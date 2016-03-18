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

#[macro_export]
macro_rules! try_destructure {
  ($r: ident) => ({
    use $crate::response::InvokeResponse;
    use $crate::destructuring::{DestructurableSet, InvokeArgSetError};

    match $r.destructure_set() {
      Ok(e) => e,
      Err(error) => {
        match error {
          InvokeArgSetError::TypeMismatch {arg_idx} => {
            return InvokeResponse::with_fail_message(format!("Argument in position [{}] did not have the correct type or was unparseable", arg_idx))
          },
          InvokeArgSetError::ArgCountMismatch {expected, actual} => {
            return InvokeResponse::with_fail_message(format!("Expected [{}] arguments, but found [{}] in step definition", expected, actual))
          }
        }
      }
    }
  })
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

  #[test]
  fn macro_one_arg_works() {
    type World = u32;
    let mut world = 0;
    let mut cuke: Cucumber<World> = Cucumber::new();
    Given!(cuke; "^I do (\\d+) basic things?$", |_, _, (count,): (u32,)| {
      InvokeResponse::check_eq(count, 1)
    });
    assert_eq!(cuke.invoke("I do 1 basic thing", &mut world, None), InvokeResponse::Success);
  }

  #[test]
  fn macro_two_args_work() {
    type World = u32;
    let mut world = 0;
    let mut cuke: Cucumber<World> = Cucumber::new();
    Given!(cuke; "^I( don't)? do (\\d+) basic things?$", |_, _, (negate, count): (Option<String>, u32)| {
      if negate.is_some() {
        InvokeResponse::check_not_eq(count, 1)
      } else {
        InvokeResponse::check_eq(count, 1)
      }
    });
    assert_eq!(cuke.invoke("I don't do 2 basic things", &mut world, None), InvokeResponse::Success);
  }

  #[test]
  fn macro_incorrect_arg_count() {
    type World = u32;
    let mut world = 0;
    let mut cuke: Cucumber<World> = Cucumber::new();
    Given!(cuke; "^I cant count my args$", |_, _, (_,): (u32,)| {
      InvokeResponse::Success
    });
    assert_eq!(cuke.invoke("I cant count my args", &mut world, None),
               InvokeResponse::with_fail_message("Expected [1] arguments, but found [0] in step definition"));
  }

  #[test]
  fn macro_unparsable_arg() {
    type World = u32;
    let mut world = 0;
    let mut cuke: Cucumber<World> = Cucumber::new();
    Given!(cuke; "^I want a number but got a \"(.*)\"$", |_, _, (_,): (u32,)| {
      InvokeResponse::Success
    });
    assert_eq!(cuke.invoke("I want a number but got a \"word\"", &mut world, None),
               InvokeResponse::with_fail_message("Argument in position [0] did not have the correct type or was unparseable"));
  }
}
