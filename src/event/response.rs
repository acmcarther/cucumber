//! Contains responses for Gherkin interpreter (or Wire Protocol).
//!
//! Consumers mostly need to be concerned with [InvokeResponse](./enum.InvokeResponse.html), as it is the
//! return type of all step defintions.

#[cfg(feature = "serde_macros")]
include!("response.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/event/response.rs"));

use serde::{self, Serializer};
use serde::ser::impls::TupleVisitor2;
use serde::ser::MapVisitor;
use std::fmt::Debug;

// NOTE: These defined in response.rs.in (as they need to derive Serialize)
// pub struct Step
// pub struct FailMessage

/// Types of responses produced by [runners](../../runner/struct.WorldRunner.html)
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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct StepArg {
  pub val: Option<String>,
  pub pos: Option<u32>
}

impl Serialize for StepArg {
  fn serialize<S: serde::ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
    s.serialize_struct("StepArg", StepArgVisitor {
      value: self,
      state: 0
    })
  }
}

struct StepArgVisitor<'a> {
  value: &'a StepArg,
  state: u8
}

impl<'a> MapVisitor for StepArgVisitor<'a> {
  fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
      where S: serde::Serializer
  {
    match self.state {
      0 => {
        self.state += 1;
        match self.value.val {
          Some(ref v) => Ok(Some(try!(serializer.serialize_struct_elt("val", v.clone())))),
          None => Ok(Some(try!(serializer.serialize_struct_elt("val", ()))))
        }
      },
      1 => {
        self.state += 1;
        match self.value.pos {
          Some(ref v) => Ok(Some(try!(serializer.serialize_struct_elt("pos", v.clone())))),
          None => Ok(Some(try!(serializer.serialize_struct_elt("pos", ()))))
        }
      },
      _ => {
        Ok(None)
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


/// The low level type capturing the possible outcomes a step invocation may have.
///
/// Typical instantiation of this type will be done using the helpers provided.
///
/// This type is designed to be heavily composable, as it is the form many operations against state
/// will take. If it doesn't suit a particular use case, that use case was probably not conceived of and should be included!
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
  /// Build an InvokeResponse::Pending with a message
  pub fn pending_from_str(val: &str) -> InvokeResponse {
    InvokeResponse::Pending(val.to_owned())
  }

  /// Build an InvokeResponse::Fail with a message
  pub fn fail_from_str(val: &str) -> InvokeResponse {
    InvokeResponse::Fail(FailMessage::new(val.to_owned()))
  }

  /// Return an InvokeResponse reflecting an equality check
  pub fn check_eq<T: PartialEq + Debug>(first: T, second: T) -> InvokeResponse {
    if first == second {
      InvokeResponse::Success
    } else {
      InvokeResponse::fail_from_str(&format!("Value [{:?}] was not equal to [{:?}]", first, second))
    }
  }

  /// Return an InvokeResponse reflecting a negative equality check
  pub fn check_not_eq<T: PartialEq + Debug>(first: T, second: T) -> InvokeResponse {
    if first == second {
      InvokeResponse::fail_from_str(&format!("Value [{:?}] was equal to [{:?}]", first, second))
    } else {
      InvokeResponse::Success
    }
  }

  /// Return an InvokeResponse reflecting a boolean outcome
  pub fn check(b: bool) -> InvokeResponse {
    if b {
      InvokeResponse::Success
    } else {
      InvokeResponse::fail_from_str("invoke response check failed")
    }
  }

  /// Return an InvokeResponse reflecting a boolean outcome with a custom message
  pub fn expect(b: bool, fail_msg: &str) -> InvokeResponse {
    if b {
      InvokeResponse::Success
    } else {
      InvokeResponse::fail_from_str(fail_msg)
    }
  }

  /// Compose InvokeResponses with "and" logic, exiting on non-success
  pub fn and(self, other: InvokeResponse) -> InvokeResponse {
    match self {
      InvokeResponse::Success => other,
      _ => self
    }
  }

  /// Compose InvokeResponses with "or" logic, exiting on success
  pub fn or(self, other: InvokeResponse) -> InvokeResponse {
    match self {
      InvokeResponse::Fail(_) | InvokeResponse::Pending(_) => other,
      _ => self
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_json;

  #[test]
  fn invoke_response_check_eq() {
    let eq = InvokeResponse::check_eq(1, 1);
    let not_eq = InvokeResponse::check_eq(1, 2);

    assert_eq!(eq, InvokeResponse::Success);
    assert_eq!(not_eq, InvokeResponse::fail_from_str("Value [1] was not equal to [2]"));
  }

  #[test]
  fn invoke_response_check_not_eq() {
    let eq = InvokeResponse::check_not_eq(1, 1);
    let not_eq = InvokeResponse::check_not_eq(1, 2);

    assert_eq!(eq, InvokeResponse::fail_from_str("Value [1] was equal to [1]"));
    assert_eq!(not_eq, InvokeResponse::Success);
  }

  #[test]
  fn invoke_response_check() {
    let t = InvokeResponse::check(true);
    let f = InvokeResponse::check(false);

    assert_eq!(t, InvokeResponse::Success);
    assert_eq!(f, InvokeResponse::fail_from_str("invoke response check failed"));
  }

  #[test]
  fn invoke_response_expect() {
    let t = InvokeResponse::expect(true, "Unevaluated message");
    let f = InvokeResponse::expect(false, "Evaluated message");

    assert_eq!(t, InvokeResponse::Success);
    assert_eq!(f, InvokeResponse::fail_from_str("Evaluated message"));
  }

  #[test]
  fn invoke_response_and() {
    // T & T = T
    assert_eq!(
      InvokeResponse::Success,
      InvokeResponse::Success.and(InvokeResponse::Success)
    );

    // T & F = F
    assert_eq!(
      InvokeResponse::fail_from_str("msg"),
      InvokeResponse::Success.and(InvokeResponse::fail_from_str("msg"))
    );

    // F & T = F
    assert_eq!(
      InvokeResponse::fail_from_str("msg"),
      InvokeResponse::fail_from_str("msg").and(InvokeResponse::Success)
    );

    // F1 & F2 = F1
    assert_eq!(
      InvokeResponse::fail_from_str("msg1"),
      InvokeResponse::fail_from_str("msg1").and(InvokeResponse::fail_from_str("msg2"))
    );
  }

  #[test]
  fn invoke_response_or() {
    // T & T = T
    assert_eq!(
      InvokeResponse::Success,
      InvokeResponse::Success.or(InvokeResponse::Success)
    );

    // T & F = T
    assert_eq!(
      InvokeResponse::Success,
      InvokeResponse::Success.or(InvokeResponse::fail_from_str("msg"))
    );

    // F & T = T
    assert_eq!(
      InvokeResponse::Success,
      InvokeResponse::fail_from_str("msg").or(InvokeResponse::Success)
    );

    // F1 & F2 = F2
    assert_eq!(
      InvokeResponse::fail_from_str("msg2"),
      InvokeResponse::fail_from_str("msg1").or(InvokeResponse::fail_from_str("msg2"))
    );
  }

  #[test]
  fn it_serializes_step_matches_no_match() {
    let response = Response::StepMatches(StepMatchesResponse::NoMatch);
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\",[]]");
  }

  #[test]
  fn it_serializes_step_matches_match() {
    let response = Response::StepMatches(StepMatchesResponse::Match(vec!(Step {id: "1".to_owned(), source: "test".to_owned(), args: vec!(StepArg { val: Some("arg".to_owned()), pos: Some(0)}) })));
    let string = serde_json::to_string(&response);
    assert_eq!(string.unwrap(), "[\"success\",[{\"id\":\"1\",\"args\":[{\"val\":\"arg\",\"pos\":0}],\"source\":\"test\"}]]");
  }


  #[test]
  fn it_serializes_invoke_pending() {
    let response = Response::Invoke(InvokeResponse::pending_from_str("stuff isn't done"));
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
    let response = Response::Invoke(InvokeResponse::fail_from_str("stuff is broken"));
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
