#[cfg(feature = "serde_macros")]
include!("response.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/cucumber/response.rs"));

use serde::{self, Serializer};
use serde::ser::impls::TupleVisitor2;
use std::fmt::Display;

// NOTE: These defined in response.rs.in (as they need to derive Serialize)
// pub struct Step
// pub struct StepArg
// pub struct FailMessage

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
  pub fn with_fail_message<T: ToString>(val: T) -> InvokeResponse {
    InvokeResponse::Fail(FailMessage::new(val.to_string()))
  }

  pub fn check_eq<T: PartialEq + Display>(first: T, second: T) -> InvokeResponse {
    if first == second {
      InvokeResponse::Success
    } else {
      InvokeResponse::with_fail_message(format!("Value [{}] was not equal to [{}]", first, second))
    }
  }

  pub fn check<T>(b: bool) -> InvokeResponse {
    if b {
      InvokeResponse::Success
    } else {
      InvokeResponse::with_fail_message("invoke response check failed")
    }
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
