use regex::Regex;
use state::Cucumber;
use event::request::{InvokeArgument, Request};
use event::response::{InvokeResponse, Response, StepMatchesResponse};
use definitions::registration::{CucumberRegistrar, SimpleStep};
use std::panic::{self, AssertUnwindSafe};

use std::str::FromStr;

/// The step runner for [Cucumber state](../state/struct.Cucumber.html)
///
/// The runner stands in for the Cucumber instance and provides an interface for
/// [Request](../event/request/enum.Request.html) events to be translated into
/// state changes and
/// step invocations, along with a
/// [Response](../event/response/enum.Response.html). These are typically
/// supplied by a running
/// [Server](../server/struct.Server.html), but may be supplied by a native
/// Gherkin implementation
/// later.
///
/// Typically this struct will only be instantiated by the user, and then
/// passed to a Server to
/// maintain.
///
#[allow(dead_code)]
pub struct WorldRunner<World> {
  cuke: Cucumber<World>,
  world: World,
}

impl<World> WorldRunner<World> {
  #[allow(dead_code)]
  pub fn new(world: World) -> WorldRunner<World> {
    WorldRunner {
      cuke: Cucumber::new(),
      world: world,
    }
  }
}

/// An interface for implementers that can consume a
/// [Request](../event/request/enum.Request.html) and yield a
/// [Response](../event/response/enum.Response.html)
///
/// This generally refers to [WorldRunner](./struct.WorldRunner.html)
pub trait CommandRunner {
  fn execute_cmd(&mut self, req: Request) -> Response;
}

impl<T: Fn(Request) -> Response> CommandRunner for T {
  fn execute_cmd(&mut self, req: Request) -> Response {
    self(req)
  }
}

impl<World> CommandRunner for WorldRunner<World> {
  fn execute_cmd(&mut self, req: Request) -> Response {
    match req {
      Request::BeginScenario(params) => {
        self.cuke.tags = params.tags;
        Response::BeginScenario
      },
      Request::Invoke(params) => {
        let step = self.cuke
          .step(u32::from_str(&params.id).unwrap())
          .unwrap();
        Response::Invoke(invoke_to_response(step, &self.cuke, &mut self.world, params.args))
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
        self.cuke.tags = Vec::new();
        Response::EndScenario
      },
      // TODO: For some reason, cucumber prints the ruby snippet too. Fix that
      Request::SnippetText(params) => {
        let text = format!("  // In a step registration block where cuke: &mut \
                            CucumberRegistrar<YourWorld>\n  use cucumber::InvokeResponse;\n  use \
                            cucumber::helpers::r;\n  {}!(cuke, r(\"^{}$\"), Box::new(move |c, _, \
                            _| {{\n    c.pending(\"TODO\")\n  }}));",
                           params.step_keyword,
                           params.step_name);

        Response::SnippetText(text)
      },
    }
  }
}

impl<World> CucumberRegistrar<World> for WorldRunner<World> {
  fn given(&mut self, file: &str, line: u32, regex: Regex, step: SimpleStep<World>) {
    self.cuke.given(file, line, regex, step)
  }

  fn when(&mut self, file: &str, line: u32, regex: Regex, step: SimpleStep<World>) {
    self.cuke.when(file, line, regex, step)
  }

  fn then(&mut self, file: &str, line: u32, regex: Regex, step: SimpleStep<World>) {
    self.cuke.then(file, line, regex, step)
  }
}

pub fn invoke_to_response<World>(test_body: &SimpleStep<World>,
                                 cuke: &Cucumber<World>,
                                 world: &mut World,
                                 args: Vec<InvokeArgument>)
                                 -> InvokeResponse {
  let result = panic::catch_unwind(AssertUnwindSafe(|| test_body(cuke, world, args)));
  match result {
    Ok(()) => InvokeResponse::Success,
    Err(err) => {
      // Yoinked from rustc libstd, with InvokeResponse added as a possible cast
      let msg = match err.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => {
          match err.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => {
              match err.downcast_ref::<InvokeResponse>() {
                Some(s) => return s.clone(),
                None => "Box<Any>",
              }
            },
          }
        },
      };
      InvokeResponse::fail_from_str(msg)
    },
  }
}
