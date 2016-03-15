use serde;

use serde::{Serialize, Serializer};
use serde::ser::impls::TupleVisitor2;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Response {
  StepMatches(StepMatchesResponse),
  Invoke(InvokeResponse),
  BeginScenario,
  EndScenario,
  SnippetText(String),
}

impl Serialize for Response {
  fn serialize<S: serde::ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
    match self {
      &Response::StepMatches(ref response) => {
        let empty_vec = Vec::new();
        let body = match response {
          &StepMatchesResponse::NoMatch => {
            &empty_vec
          },
          &StepMatchesResponse::Match(ref steps) => {
            steps
          },
        };
        s.serialize_seq(TupleVisitor2::new(&("success", body)))
      },
      &Response::Invoke(ref response) => {
        match response {
          &InvokeResponse::Pending(ref message) => {
            s.serialize_seq(TupleVisitor2::new(&("pending", message)))
          },
          &InvokeResponse::Success => {
            s.serialize_seq(Some(&("success")))
          },
          &InvokeResponse::Fail(ref message) => {
            s.serialize_seq(TupleVisitor2::new(&("fail", message)))
          },
        }
      },
      &Response::BeginScenario => {
        s.serialize_seq(Some(&("success")))
      },
      &Response::EndScenario => {
        s.serialize_seq(Some(&("success")))
      },
      &Response::SnippetText(ref text) => {
        s.serialize_seq(TupleVisitor2::new(&("success", text.clone())))
      },
    }
  }
}
// ["success", []"]
// ["success", []"]
// ["success", [{"id": "1", "args":[]]
// ["success", [{"id": "1", "args":[{"val": "wired", "pos": 10}]}]]
// https://www.relishapp.com/cucumber/cucumber/docs/wire-protocol/invoke-message
#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum StepMatchesResponse {
  NoMatch,
  Match(Vec<Step>)
}

#[derive(Serialize, Eq, PartialEq, Clone, Debug)]
pub struct Step {
  pub id: String,
  pub args: Vec<StepArg>,
  pub source: String,
}

#[derive(Serialize, Eq, PartialEq, Clone, Debug)]
pub struct StepArg {
  pub val: String,
  pub pos: u32
}

// ["pending", "I'll do it later"]
// ["success"]
// ["fail", {"message": "The wires are down", "exception": "Some.Foreign.ExceptionType"}]
#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum InvokeResponse {
  Pending(String),
  Success,
  Fail(FailMessage)
}

impl InvokeResponse {
  pub fn pending<T: ToString>(val: T) -> InvokeResponse {
    InvokeResponse::Pending(val.to_string())
  }
  pub fn fail<T: ToString>(val: T) -> InvokeResponse {
    InvokeResponse::Fail(FailMessage::new(val.to_string()))
  }
}

#[derive(Serialize, Eq, PartialEq, Clone, Debug)]
pub struct FailMessage {
  message: String,
  exception: String
}

impl FailMessage {
  pub fn new(str: String) -> FailMessage{
    FailMessage { message: str, exception: "".to_owned() }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_json;

  #[test]
  fn it_serializes_step_matches_no_match() {
    let response = Response::StepMatches(StepMatchesResponse::NoMatch);
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\",[]]");
  }

  #[test]
  fn it_serializes_step_matches_match() {
    let response = Response::StepMatches(StepMatchesResponse::Match(vec!(Step {id: "1".to_owned(), source: "test".to_owned(), args: vec!(StepArg { val: "arg".to_owned(), pos: 0}) })));
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\",[{\"id\":\"1\",\"args\":[{\"val\":\"arg\",\"pos\":0}],\"source\":\"test\"}]]");
  }


  #[test]
  fn it_serializes_invoke_pending() {
    let response = Response::Invoke(InvokeResponse::Pending("stuff isn't done".to_owned()));
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"pending\",\"stuff isn't done\"]");
  }

  #[test]
  fn it_serializes_invoke_success() {
    let response = Response::Invoke(InvokeResponse::Success);
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\"]");
  }

  #[test]
  fn it_serializes_invoke_fail() {
    let response = Response::Invoke(InvokeResponse::Fail(FailMessage{ message: "stuff is broken".to_owned(), exception: "".to_owned()}));
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"fail\",{\"message\":\"stuff is broken\",\"exception\":\"\"}]");
  }

  #[test]
  fn it_serializes_begin_scenario() {
    let response = Response::BeginScenario;
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\"]");
  }

  #[test]
  fn it_serializes_end_scenario() {
    let response = Response::EndScenario;
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\"]");
  }

  #[test]
  fn it_serializes_snippet_text() {
    let response = Response::SnippetText("Snippet".to_owned());
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\",\"Snippet\"]");
  }
}
