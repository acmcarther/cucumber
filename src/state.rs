pub use external_regex::{Regex, Captures};

use std::collections::HashMap;

use response::StepArg;
use response::Step as ResponseStep;
use definitions::{Step, StepId};

pub struct CucumberState {
  pub tags: Vec<String>
}

impl CucumberState {
  pub fn new() -> CucumberState {
    CucumberState { tags: Vec::new() }
  }
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn cuke_instantiates() {
    type World = u32;

    let _: Cucumber<World> = Cucumber::new();
  }
}
