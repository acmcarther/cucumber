pub use regex::{Captures, Regex};

use std::collections::HashMap;

use event::response::InvokeResponse;
use event::response::StepArg;
use event::response::Step as ResponseStep;
use event::request::InvokeArgument;
use definitions::registration::SimpleStep;

/// The trait steps must implement to be invokable
///
/// As far as I can tell, this is unimplementable because of the blanket impl
/// Generally, any closure closing over sendable data, and taking sendable
/// arguments in the form
/// indicated should be implemented automatically by the blanket impl
pub trait SendableStep<World>
  : Send + Fn(&Cucumber<World>, &mut World, Vec<InvokeArgument>) -> InvokeResponse
  {
}

impl<T, World> SendableStep<World> for T
  where T: Send + Fn(&Cucumber<World>, &mut World, Vec<InvokeArgument>) -> InvokeResponse
{
}

pub type Step<World> = Box<SendableStep<World, Output = InvokeResponse>>;

pub type StepId = u32;

/// The Cucumber state wrapper
///
/// This struct maintains the list of primitive step components, and does the
/// lookups to find and
/// execute steps. It also maintains the current tags
///
/// This struct is typically only used directly when invoking steps from other
/// steps, as in the
/// example. Otherwise, it is managed by the
/// [WorldRunner](../runner/struct.WorldRunner.html)
///
/// # Example
///
/// ```
/// use cucumber::{
///   cucumber_regex,
///   Cucumber
/// };
///
/// fn main() {
///   let mut cuke: Cucumber<u32> = Cucumber::new();
///   cuke.insert_step("dummy_path".to_owned(),
/// cucumber_regex::build("^test$"), Box::new(move |c: &Cucumber<u32>,
/// world:
///     &mut u32, _| {
///     // Undefined step here will return a "no match" error
///     c.invoke("another step", world, None)
///   }));
/// }
/// ```
///
pub struct Cucumber<World> {
  step_regexes: Vec<Regex>,
  step_ids: HashMap<String, (StepId, String)>,
  steps: HashMap<StepId, SimpleStep<World>>,
  pub tags: Vec<String>,
}

impl<World> Cucumber<World> {
  pub fn new() -> Cucumber<World> {
    Cucumber {
      step_regexes: Vec::new(),
      step_ids: HashMap::new(),
      steps: HashMap::new(),
      tags: Vec::new(),
    }
  }

  /// Add a new step to the set of steps.
  ///
  /// This method is typically executed by a
  /// [WorldRunner](../runner/struct.WorldRunner.html) when
  /// `#given`, `#when` or `#then` methods are called.
  pub fn insert_step(&mut self, path: String, regex: Regex, step: SimpleStep<World>) {
    let str_rep = regex.as_str().to_owned();
    self.step_regexes.push(regex);

    let this_id = self.step_ids.values().max().map(|&(ref res, _)| res + 1).unwrap_or(0);
    // TODO: handle existing str_reps in hash
    self.step_ids.insert(str_rep, (this_id.clone(), path));

    self.steps.insert(this_id, step);
  }

  /// Find a step or steps matching a given string.
  ///
  /// This method is typically executed by a
  /// [WorldRunner](../runner/struct.WorldRunner.html) when
  /// trying to find a step corresponding to a string provided by the
  /// [Server](../server/struct.Server.html).
  pub fn find_match(&self, str: &str) -> Vec<ResponseStep> {
    self.step_regexes.iter()
      .filter_map(|ref regex| {
        // Get captures from regex
        regex.captures(str).map(|captures| {
          let captures: Vec<StepArg> =
            captures
              .iter_pos()  // Iterate over byte idx
              .enumerate() // Get simple idx -- captures.at uses simple idx, while cuke needs byte idx
              .skip(1)     // Ignore the match against the entire string
              .map(|(idx, pos)| {
                let pos = pos.map(|(begin_idx,_)| begin_idx as u32);
                StepArg { pos: pos, val: captures.at(idx).map(|v| v.to_owned()) }
              })
              .collect();
          let (id, path) = self.step_ids.get(regex.as_str()).unwrap().clone();
          ResponseStep {id: id.to_string(), args: captures, source: path }
        })
      })
      .collect()
  }

  /// Set a step pending with a useful message
  pub fn pending(&self, message: &str) {
    panic!(InvokeResponse::pending_from_str(message));
  }

  /// Fails with a useful message
  pub fn fail(&self, message: &str) {
    panic!(InvokeResponse::fail_from_str(message));
  }

  /// Ends step execution successfully.
  /// I have no idea why anyone would want to do this, but it fills out the
  /// "pending, fail,
  /// success" set.
  pub fn succeed_immediately(&self) {
    panic!(InvokeResponse::Success);
  }

  /// Directly execute the step matched by a regular expression.
  ///
  /// The most typical method applied on a Cucumber instance, this method
  /// allows steps to invoke
  /// other steps. The final argument is for use with docstring arguments or
  /// tables.
  pub fn invoke(&self, str: &str, world: &mut World, extra_arg: Option<InvokeArgument>) {
    let mut matches = self.find_match(str);
    match matches.len() {
      0 => panic!("Direct invoke matched no steps"),
      1 => {
        let response_step = matches.pop().unwrap();
        let mut invoke_args: Vec<InvokeArgument> = response_step.args
          .into_iter()
          .map(|arg| InvokeArgument::from_step_arg(arg))
          .collect();

        if extra_arg.is_some() {
          invoke_args.push(extra_arg.unwrap());
        }

        self.step(response_step.id
            .parse()
            .unwrap())
          .unwrap()(&self, world, invoke_args)
      },
      _ => panic!("Direct invoke matched more than one step"),
    }
  }

  /// Retrieve a step based on its Id
  ///
  /// This method is typically executed by a
  /// [WorldRunner](../runner/struct.WorldRunner.html) when
  /// a request specifically invokes a step by Id. This is typical of the
  /// Cucumber Wire protocol.
  pub fn step(&self, id: StepId) -> Option<&SimpleStep<World>> {
    self.steps.get(&id)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use cucumber_regex as regex;
  use event::response::StepArg;
  use event::response::Step as ResponseStep;

  #[test]
  fn cuke_instantiates() {
    type World = u32;

    let _: Cucumber<World> = Cucumber::new();
  }

  #[test]
  fn cuke_inserts_step() {
    type World = u32;

    let mut cucumber: Cucumber<World> = Cucumber::new();
    cucumber.insert_step("file:line".to_owned(),
                         regex::build("^example$"),
                         Box::new(|_, _, _| ()));
  }

  #[test]
  fn cuke_invokes() {
    type World = u32;

    let mut world = 0;

    let mut cucumber: Cucumber<World> = Cucumber::new();
    cucumber.insert_step("file:line".to_owned(),
                         regex::build("^example$"),
                         Box::new(|_, _, _| ()));
    cucumber.invoke("example", &mut world, None);
  }

  #[test]
  #[should_panic(expected = "Direct invoke matched more than one step")]
  fn cuke_invoke_fails_on_multiple_match() {
    type World = u32;

    let mut world = 0;

    let mut cucumber: Cucumber<World> = Cucumber::new();
    cucumber.insert_step("file:line".to_owned(),
                         regex::build("^example$"),
                         Box::new(|_, _, _| ()));
    cucumber.insert_step("file:line".to_owned(),
                         regex::build("^ex"),
                         Box::new(|_, _, _| ()));
    cucumber.invoke("example", &mut world, None);
  }

  #[test]
  #[should_panic(expected = "Direct invoke matched no steps")]
  fn cuke_invoke_fails_on_no_match() {
    type World = u32;

    let mut world = 0;

    let cucumber: Cucumber<World> = Cucumber::new();
    cucumber.invoke("example", &mut world, None)
  }

  #[test]
  fn find_match_optional_args_work() {
    type World = u32;

    let mut cucumber: Cucumber<World> = Cucumber::new();
    cucumber.insert_step("file:line".to_owned(),
                         regex::build("^example( stuff)? (\\d+)$"),
                         Box::new(|_, _, _| ()));
    {
      let mut step_matches = cucumber.find_match("example 5");
      assert_eq!(step_matches.len(), 1);
      let step_details = step_matches.pop().unwrap();
      assert_eq!(step_details,
                 ResponseStep {
                   id: "0".to_owned(),
                   args: vec![StepArg {
                                val: None,
                                pos: None,
                              },
                              StepArg {
                                val: Some("5".to_owned()),
                                pos: Some(8),
                              }],
                   source: "file:line".to_owned(),
                 })
    }
    {
      let mut step_matches = cucumber.find_match("example stuff 5");
      assert_eq!(step_matches.len(), 1);
      let step_details = step_matches.pop().unwrap();
      assert_eq!(step_details,
                 ResponseStep {
                   id: "0".to_owned(),
                   args: vec![StepArg {
                                val: Some(" stuff".to_owned()),
                                pos: Some(7),
                              },
                              StepArg {
                                val: Some("5".to_owned()),
                                pos: Some(14),
                              }],
                   source: "file:line".to_owned(),
                 })
    }
  }
}
