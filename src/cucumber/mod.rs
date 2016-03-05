pub use regex::{Regex, Captures};
use std::collections::HashMap;

pub mod helpers;
mod request;
mod response;

pub use self::request::*;
pub use self::response::{Response, InvokeResponse, StepMatchesResponse, StepArg, FailMessage};
pub use self::response::Step as ResponseStep;

pub struct CucumberState {
  pub tags: Vec<String>
}

impl CucumberState {
  pub fn new() -> CucumberState {
    CucumberState { tags: Vec::new() }
  }
}

pub trait SendableStep<World>: Send + Fn(&mut CucumberState, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}
impl<T, World> SendableStep<World> for T where T: Send + Fn(&mut CucumberState, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}

pub type Step<World> = Box<SendableStep<World, Output=InvokeResponse>>;

pub type StepId = u32;

pub trait CucumberRegistrar<World> {
  fn given(&mut self, file: &str, line: u32, Regex, Step<World>);
  fn when(&mut self, file: &str, line: u32, Regex, Step<World>);
  fn then(&mut self, file: &str, line: u32, Regex, Step<World>);
}

pub struct Cucumber<World> {
  step_regexes: Vec<Regex>,
  step_ids: HashMap<String, (StepId, String)>,
  steps: HashMap<StepId, Step<World>>
}

impl <World> Cucumber<World> {

  pub fn new() -> Cucumber<World> {
    Cucumber {
      step_regexes: Vec::new(),
      step_ids: HashMap::new(),
      steps: HashMap::new()
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
    // TODO: Detangle this
    self.step_regexes.iter()
      .filter_map(|ref regex| {
        // Get captures from regex
        regex.captures(str).map(|captures| {
          let captures: Vec<StepArg> =
            captures
              .iter_pos()  // Iterate over byte idx
              .enumerate() // Get simple idx -- captures.at uses simple idx, while cuke needs byte idx
              .skip(1)     // Ignore the match against the entire string
              .filter_map(|(idx, pos)| pos.map(|(begin_idx, _)| {
                StepArg { pos: begin_idx as u32, val: captures.at(idx).unwrap().to_owned() }
              }))
              .collect();
          let (id, path) = self.step_ids.get(regex.as_str()).unwrap().clone();
          ResponseStep {id: id.to_string(), args: captures, source: path }
        })
      })
      .collect()
  }

  pub fn step(&self, id: StepId) -> Option<&Step<World>> {
    self.steps.get(&id)
  }
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
  use super::*;
  use super::helpers::*;

  #[test]
  fn cuke_instantiates() {
    type World = u32;

    let _: Cucumber<World> = Cucumber::new();
  }

  #[test]
  fn cuke_add_step() {
    type World = u32;
    let mut cuke: Cucumber<World> = Cucumber::new();
    cuke.given(file!(), line!(), r("^I do a basic thing$"), Box::new(move |_, _, _| InvokeResponse::Success));
  }

  #[test]
  fn cuke_find_match() {
    type World = u32;
    let mut cuke: Cucumber<World> = Cucumber::new();
    cuke.given("file", 0, r("^I do (\\d+) basic things?$"), Box::new(move |_, _, _| InvokeResponse::Success));

    let mut matches = cuke.find_match("I do 6 basic things");
    assert!(matches.len() == 1);
    let first_match = matches.pop().unwrap();
    assert_eq!(first_match, ResponseStep {id: "0".to_owned(), source: "file:0".to_owned(), args: vec!(StepArg { pos: 5, val: "6".to_owned()}) });
  }
}
