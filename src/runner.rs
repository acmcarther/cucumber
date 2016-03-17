use cucumber::{Step, Regex, Cucumber, CucumberRegistrar, CucumberState};
use cucumber::{Request, Response, StepMatchesResponse};

use std::str::FromStr;

#[allow(dead_code)]
pub struct WorldRunner<World> {
  cuke: Cucumber<World>,
  cuke_state: CucumberState,
  world: World,
}

impl <World> WorldRunner<World> {
  #[allow(dead_code)]
  pub fn new(world: World) -> WorldRunner<World> {
    WorldRunner {
      cuke: Cucumber::new(),
      cuke_state: CucumberState::new(),
      world: world,
    }
  }
}

pub trait CommandRunner {
  fn execute_cmd(&mut self, req: Request) -> Response;
}

impl <T: Fn(Request) -> Response> CommandRunner for T {
  fn execute_cmd(&mut self, req: Request) -> Response {
    self(req)
  }
}

impl <World> CommandRunner for WorldRunner<World> {
  fn execute_cmd(&mut self, req: Request) -> Response {
    match req {
      Request::BeginScenario(params) => {
        self.cuke_state.tags = params.tags;
        Response::BeginScenario
      },
      Request::Invoke(params) => {
        let step = self.cuke.step(u32::from_str(&params.id).unwrap()).unwrap();
        Response::Invoke(step(&mut self.cuke_state, &mut self.world, params.args))
      },
      Request::StepMatches(params) => {
        let matches = self.cuke.find_match(&params.name_to_match);
        if matches.len() == 0 {
          Response::StepMatches(StepMatchesResponse::NoMatch)
        } else {
          Response::StepMatches(StepMatchesResponse::Match(matches))
        }
      },
      Request::EndScenario(_) => {
        self.cuke_state.tags = Vec::new();
        Response::EndScenario
      },
      // TODO: For some reason, cucumber prints the ruby snippet too. Fix that
      Request::SnippetText(params) => {
        let text =
          format!("  // In a step registration block where cuke: &mut CucumberRegistrar<YourWorld>\
          \n  use cucumber::InvokeResponse;\
          \n  use cucumber::helpers::r;\
          \n  {}!(cuke, r(\"^{}$\"), Box::new(move |_, _, _| {{\
          \n    InvokeResponse::pending(\"TODO\")\
          \n  }}));\
          ", params.step_keyword, params.step_name);

        Response::SnippetText(text)
      }
    }
  }
}

impl <World> CucumberRegistrar<World> for WorldRunner<World> {
  fn given(&mut self, file: &str, line: u32, regex: Regex, step: Step<World>) {
    self.cuke.given(file, line, regex, step)
  }

  fn when(&mut self, file: &str, line: u32, regex: Regex, step: Step<World>) {
    self.cuke.when(file, line, regex, step)
  }

  fn then(&mut self, file: &str, line: u32, regex: Regex, step: Step<World>) {
    self.cuke.then(file, line, regex, step)
  }
}


#[cfg(test)]
mod test {
  use super::*;
  use cucumber::CucumberRegistrar;
  use cucumber::InvokeResponse;

  use regex;

  #[test]
  fn runner_instantiates() {
    let _: WorldRunner<u32> = WorldRunner::new(0);
  }

  #[test]
  fn runner_registers_steps() {
    let world: u32 = 0;
    let mut runner = WorldRunner::new(world);

    runner.when(file!(), line!(), regex::build("^I increment my world$"), Box::new(move |_, world, _| {
      *world = *world + 1;
      InvokeResponse::Success
    }));
  }

}
