#[cfg(feature = "serde_macros")]
include!("response.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/response.rs"));

use serde::{self, Serializer};
use serde::ser::impls::TupleVisitor2;
use serde::ser::MapVisitor;
use std::fmt::Debug;

// NOTE: These defined in response.rs.in (as they need to derive Serialize)
// pub struct Step
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

  pub fn check_eq<T: PartialEq + Debug>(first: T, second: T) -> InvokeResponse {
    if first == second {
      InvokeResponse::Success
    } else {
      InvokeResponse::with_fail_message(format!("Value [{:?}] was not equal to [{:?}]", first, second))
    }
  }

  pub fn check_not_eq<T: PartialEq + Debug>(first: T, second: T) -> InvokeResponse {
    if first == second {
      InvokeResponse::with_fail_message(format!("Value [{:?}] was equal to [{:?}]", first, second))
    } else {
      InvokeResponse::Success
    }
  }

  pub fn check(b: bool) -> InvokeResponse {
    if b {
      InvokeResponse::Success
    } else {
      InvokeResponse::with_fail_message("invoke response check failed")
    }
  }

  pub fn expect(b: bool, fail_msg: &str) -> InvokeResponse {
    if b {
      InvokeResponse::Success
    } else {
      InvokeResponse::with_fail_message(fail_msg)
    }
  }

  pub fn and(self, other: InvokeResponse) -> InvokeResponse {
    match self {
      InvokeResponse::Success => other,
      _ => self
    }
  }

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
    assert_eq!(not_eq, InvokeResponse::with_fail_message("Value [1] was not equal to [2]"));
  }

  #[test]
  fn invoke_response_check_not_eq() {
    let eq = InvokeResponse::check_not_eq(1, 1);
    let not_eq = InvokeResponse::check_not_eq(1, 2);

    assert_eq!(eq, InvokeResponse::with_fail_message("Value [1] was equal to [1]"));
    assert_eq!(not_eq, InvokeResponse::Success);
  }

  #[test]
  fn invoke_response_check() {
    let t = InvokeResponse::check(true);
    let f = InvokeResponse::check(false);

    assert_eq!(t, InvokeResponse::Success);
    assert_eq!(f, InvokeResponse::with_fail_message("invoke response check failed"));
  }

  #[test]
  fn invoke_response_expect() {
    let t = InvokeResponse::expect(true, "Unevaluated message");
    let f = InvokeResponse::expect(false, "Evaluated message");

    assert_eq!(t, InvokeResponse::Success);
    assert_eq!(f, InvokeResponse::with_fail_message("Evaluated message"));
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
      InvokeResponse::with_fail_message("msg"),
      InvokeResponse::Success.and(InvokeResponse::with_fail_message("msg"))
    );

    // F & T = F
    assert_eq!(
      InvokeResponse::with_fail_message("msg"),
      InvokeResponse::with_fail_message("msg").and(InvokeResponse::Success)
    );

    // F1 & F2 = F1
    assert_eq!(
      InvokeResponse::with_fail_message("msg1"),
      InvokeResponse::with_fail_message("msg1").and(InvokeResponse::with_fail_message("msg2"))
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
      InvokeResponse::Success.or(InvokeResponse::with_fail_message("msg"))
    );

    // F & T = T
    assert_eq!(
      InvokeResponse::Success,
      InvokeResponse::with_fail_message("msg").or(InvokeResponse::Success)
    );

    // F1 & F2 = F2
    assert_eq!(
      InvokeResponse::with_fail_message("msg2"),
      InvokeResponse::with_fail_message("msg1").or(InvokeResponse::with_fail_message("msg2"))
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
