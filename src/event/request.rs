//! Contains requests made by Gherkin interpreter (or Wire Protocol).
//!
//! Consumers will interact with [InvokeArgument](./enum.InvokeArgument.html) if using
//! [Cucumber's](../../state/struct.Cucumber.html) direct invoke capability with tables or
//! docstrings.

#[cfg(feature = "serde_macros")]
include!("request.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/event/request.rs"));

use std::ascii::AsciiExt;

use serde::Deserializer;
use serde::de::{SeqVisitor, Visitor};
use serde::de::impls::VecVisitor;
use serde::Error as SerdeError;

use event::response::StepArg;

// NOTE: These defined in request.rs.in (as they need to derive Deserialize)
// pub struct StepMatchesRequest
// pub struct InvokeRequest
// pub struct BeginScenarioRequest
// pub struct EndScenarioRequest
// pub struct SnippetTextRequest

/// Types of requests handled by [runners](../../runner/struct.WorldRunner.html)
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Request {
  StepMatches(StepMatchesRequest),
  Invoke(InvokeRequest),
  BeginScenario(BeginScenarioRequest),
  EndScenario(EndScenarioRequest),
  SnippetText(SnippetTextRequest),
}

impl Deserialize for Request {
  fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
    d.deserialize(RequestVisitor)
  }
}


struct RequestVisitor;

impl Visitor for RequestVisitor {
  type Value = Request;

  fn visit_seq<V: SeqVisitor>(&mut self, mut _visitor: V) -> Result<Request, V::Error> {
    let cmd_type = try!(_visitor.visit()).map(|val: String| val.to_ascii_lowercase());

    match cmd_type {
      None => Err(V::Error::invalid_length(0)),
      Some(command) => {
        match command.as_ref(){
          "step_matches" => {
            let payload = try!(_visitor.visit::<StepMatchesRequest>());
            try!(_visitor.end());
            match payload {
              None => Err(V::Error::invalid_length(1)),
              Some(payload) => Ok(Request::StepMatches(payload))
            }
          },
          "invoke" => {
            let payload = try!(_visitor.visit::<InvokeRequest>());
            try!(_visitor.end());
            match payload {
              None => Err(V::Error::invalid_length(1)),
              Some(payload) => Ok(Request::Invoke(payload))
            }
          },
          "begin_scenario" => {
            let payload = try!(_visitor.visit::<BeginScenarioRequest>());
            try!(_visitor.end());
            match payload {
              None => Ok(Request::BeginScenario(BeginScenarioRequest { tags: Vec::new() })),
              Some(payload) => Ok(Request::BeginScenario(payload))
            }
          },
          "end_scenario" => {
            let payload = try!(_visitor.visit::<EndScenarioRequest>());
            try!(_visitor.end());
            match payload {
              None => Ok(Request::EndScenario(EndScenarioRequest { tags: Vec::new() })),
              Some(payload) => Ok(Request::EndScenario(payload))
            }
          },
          "snippet_text" => {
            let payload = try!(_visitor.visit::<SnippetTextRequest>());
            try!(_visitor.end());
            match payload {
              None => Err(V::Error::invalid_length(1)),
              Some(payload) => Ok(Request::SnippetText(payload))
            }
          },
          _ => Err(V::Error::custom("Unknown command type as first value"))
        }
      },
    }
  }
}

/// The low level type capturing the possible values a step may provide.
///
/// Normal regex arguments as well as docstrings come in the form of the String variant. Conversion
/// to other types is done at later stages. Tables are represented as `Vec<Vec<String>>`
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum InvokeArgument {
  String(String),
  None,
  Table(Vec<Vec<String>>)
}

impl InvokeArgument {
  pub fn from_str(arg: &str) -> InvokeArgument {
    InvokeArgument::String(arg.to_owned())
  }

  pub fn from_step_arg(arg: StepArg) -> InvokeArgument {
    match arg.val {
      Some(v) => InvokeArgument::String(v),
      None => InvokeArgument::None
    }
  }
}

impl Deserialize for InvokeArgument {
  fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
    d.deserialize(InvokeArgumentVisitor)
  }
}

struct InvokeArgumentVisitor;

impl Visitor for InvokeArgumentVisitor {
  type Value = InvokeArgument;

  fn visit_str<E: SerdeError>(&mut self, v: &str) -> Result<InvokeArgument, E> {
    Ok(InvokeArgument::from_str(v))
  }

  fn visit_unit<E: SerdeError>(&mut self) -> Result<InvokeArgument, E> {
    Ok(InvokeArgument::None)
  }

  fn visit_seq<V: SeqVisitor>(&mut self, _visitor: V) -> Result<InvokeArgument, V::Error> {
    VecVisitor::new().visit_seq(_visitor).map(|res| InvokeArgument::Table(res))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_json;

  #[test]
  fn read_step_matches() {
    let json = "[\"step_matches\", {\"name_to_match\": \"we're all wired\"}]";
    let res = serde_json::from_str(json);
    match res.unwrap() {
      Request::StepMatches(payload) => {
        assert_eq!(payload, StepMatchesRequest {name_to_match: "we're all wired".to_owned()})
      },
      _ => panic!("result was not StepMatches type")
    }
  }

  #[test]
  fn read_invoke_no_args() {
    let json = "[\"invoke\", {\"id\":\"1\", \"args\": []}]";
    let res = serde_json::from_str(json);
    match res.unwrap() {
      Request::Invoke(payload) => {
        assert_eq!(payload, InvokeRequest {id: "1".to_owned(), args: Vec::new()})
      },
      _ => panic!("result was not Invoke type")
    }
  }

  #[test]
  fn read_invoke_string_arg() {
    let json = "[\"invoke\", {\"id\":\"1\", \"args\": [\"wired\"]}]";
    let res = serde_json::from_str(json);
    println!("{:?}", res);
    match res.unwrap() {
      Request::Invoke(payload) => {
        assert_eq!(payload, InvokeRequest {id: "1".to_owned(), args: vec!(InvokeArgument::from_str("wired"))})
      },
      _ => panic!("result was not Invoke type")
    }
  }

  #[test]
  fn read_invoke_complicated_args() {
    let json = "[\"invoke\", {\"id\":\"1\", \"args\": [\"we're\", [[\"wired\"],[\"high\"],[\"happy\"]]]}]";
    let res = serde_json::from_str(json);
    println!("{:?}", res);
    match res.unwrap() {
      Request::Invoke(payload) => {
        assert_eq!(payload, InvokeRequest {id: "1".to_owned(), args: vec!(
              InvokeArgument::from_str("we're"),
              InvokeArgument::Table(vec!(vec!("wired".to_owned()), vec!("high".to_owned()), vec!("happy".to_owned())))
              )})
      },
      _ => panic!("result was not Invoke type")
    }
  }

  #[test]
  fn read_begin_scenario_empty() {
    let json = "[\"begin_scenario\"]";
    let res = serde_json::from_str(json);
    match res.unwrap() {
      Request::BeginScenario(payload) => {
        assert_eq!(payload, BeginScenarioRequest {tags: Vec::new()})
      },
      _ => panic!("result was not BeginScenario type")
    }
  }

  #[test]
  fn read_begin_scenario() {
    let json = "[\"begin_scenario\", { \"tags\": [\"hello\"] }]";
    let res = serde_json::from_str(json);
    match res.unwrap() {
      Request::BeginScenario(payload) => {
        assert_eq!(payload, BeginScenarioRequest {tags: vec!("hello".to_owned())})
      },
      _ => panic!("result was not BeginScenario type")
    }
  }

  #[test]
  fn read_end_scenario_empty() {
    let json = "[\"end_scenario\"]";
    let res = serde_json::from_str(json);
    match res.unwrap() {
      Request::EndScenario(payload) => {
        assert_eq!(payload, EndScenarioRequest {tags: Vec::new()})
      },
      _ => panic!("result was not EndScenario type")
    }
  }

  #[test]
  fn read_end_scenario() {
    let json = "[\"end_scenario\", { \"tags\": [\"hello\"]}]";
    let res = serde_json::from_str(json);
    match res.unwrap() {
      Request::EndScenario(payload) => {
        assert_eq!(payload, EndScenarioRequest {tags: vec!("hello".to_owned())})
      },
      _ => panic!("result was not EndScenario type")
    }
  }

  #[test]
  fn read_snippet_text() {
    let json = "[\"snippet_text\", {\"step_keyword\": \"Given\", \"multiline_arg_class\":\"\", \"step_name\":\"we're all wired\"}]";
    let res = serde_json::from_str(json);
    match res.unwrap() {
      Request::SnippetText(payload) => {
        assert_eq!(payload, SnippetTextRequest {step_keyword: "Given".to_owned(), multiline_arg_class: "".to_owned(), step_name: "we're all wired".to_owned()})
      },
      _ => panic!("result was not SnippetText type")
    }
  }
}
