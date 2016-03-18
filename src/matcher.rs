pub use external_regex::{Regex, Captures};

use std::collections::HashMap;

use response::InvokeResponse;
use response::StepArg;
use response::Step as ResponseStep;
use request::InvokeArgument;
use definitions::{Step, StepId};

pub struct Matcher<World> {
  step_regexes: Vec<Regex>,
  step_ids: HashMap<String, (StepId, String)>,
  steps: HashMap<StepId, Step<World>>,
  pub tags: Vec<String>
}

impl <World> Matcher<World> {

  pub fn new() -> Matcher<World> {
    Matcher {
      step_regexes: Vec::new(),
      step_ids: HashMap::new(),
      steps: HashMap::new(),
      tags: Vec::new()
    }
  }

  pub fn insert_step(&mut self, path: String, regex: Regex, step: Step<World>) {
    let str_rep = regex.as_str().to_owned();
    self.step_regexes.push(regex);

    let this_id = self.step_ids.values().max().map(|&(ref res, _)| res + 1).unwrap_or(0);
    // TODO: handle existing str_reps in hash
    self.step_ids.insert(str_rep, (this_id.clone(), path));

    self.steps.insert(this_id, step);
  }

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

  pub fn invoke(&self, str: &str, world: &mut World, extra_arg: Option<InvokeArgument>) -> InvokeResponse {
    let mut matches = self.find_match(str);
    match matches.len() {
      0 => InvokeResponse::with_fail_message("Direct invoke matched no steps"),
      1 => {
        let response_step = matches.pop().unwrap();
        let mut invoke_args: Vec<InvokeArgument> = response_step.args.into_iter()
          .map(|arg| InvokeArgument::from_step_arg(arg))
          .collect();

        if extra_arg.is_some() {
          invoke_args.push(extra_arg.unwrap());
        }

        self.step(response_step.id.parse().unwrap()).unwrap()(&self, world, invoke_args)
      },
      _ => InvokeResponse::with_fail_message("Direct invoke matched more than one step")
    }
  }

  pub fn step(&self, id: StepId) -> Option<&Step<World>> {
    self.steps.get(&id)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use regex;
  use response::{InvokeResponse, StepArg};
  use response::Step as ResponseStep;

  #[test]
  fn matcher_instantiates() {
    type World = u32;

    let _: Matcher<World> = Matcher::new();
  }

  #[test]
  fn matcher_inserts_step() {
    type World = u32;

    let mut matcher: Matcher<World> = Matcher::new();
    matcher.insert_step("file:line".to_owned(), regex::build("^example$"), Box::new(|_, _, _| { InvokeResponse::Success }));
  }

  #[test]
  fn matcher_invokes() {
    type World = u32;

    let mut world = 0;

    let mut matcher: Matcher<World> = Matcher::new();
    matcher.insert_step("file:line".to_owned(), regex::build("^example$"), Box::new(|_, _, _| { InvokeResponse::Success }));
    assert_eq!(matcher.invoke("example", &mut world, None), InvokeResponse::Success);
  }

  #[test]
  fn matcher_invoke_fails_on_multiple_match() {
    type World = u32;

    let mut world = 0;

    let mut matcher: Matcher<World> = Matcher::new();
    matcher.insert_step("file:line".to_owned(), regex::build("^example$"), Box::new(|_, _, _| { InvokeResponse::Success }));
    matcher.insert_step("file:line".to_owned(), regex::build("^ex"), Box::new(|_, _, _| { InvokeResponse::Success }));
    assert_eq!(matcher.invoke("example", &mut world, None), InvokeResponse::with_fail_message("Direct invoke matched more than one step"));
  }

  #[test]
  fn matcher_invoke_fails_on_no_match() {
    type World = u32;

    let mut world = 0;

    let matcher: Matcher<World> = Matcher::new();
    assert_eq!(matcher.invoke("example", &mut world, None), InvokeResponse::with_fail_message("Direct invoke matched no steps"));
  }

  #[test]
  fn matcher_match_optional_args_work() {
    type World = u32;

    let mut matcher: Matcher<World> = Matcher::new();
    matcher.insert_step("file:line".to_owned(), regex::build("^example( stuff)? (\\d+)$"), Box::new(|_, _, _| { InvokeResponse::Success }));
    {
      let mut step_matches = matcher.find_match("example 5");
      assert_eq!(step_matches.len(), 1);
      let step_details = step_matches.pop().unwrap();
      assert_eq!(step_details, ResponseStep {
        id: "0".to_owned(),
        args: vec![
          StepArg {val: None, pos: None},
          StepArg {val: Some("5".to_owned()), pos: Some(8)}
        ],
        source: "file:line".to_owned()
      })
    }
    {
      let mut step_matches = matcher.find_match("example stuff 5");
      assert_eq!(step_matches.len(), 1);
      let step_details = step_matches.pop().unwrap();
      assert_eq!(step_details, ResponseStep {
        id: "0".to_owned(),
        args: vec![
          StepArg {val: Some(" stuff".to_owned()), pos: Some(7)},
          StepArg {val: Some("5".to_owned()), pos: Some(14)}
        ],
        source: "file:line".to_owned()
      })
    }
  }
}
