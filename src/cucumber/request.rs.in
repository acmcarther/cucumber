use std::ascii::AsciiExt;

use serde::{Deserialize, Deserializer};
use serde::de::{SeqVisitor, Visitor};
use serde::de::impls::VecVisitor;
use serde::Error as SerdeError;

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

// ["step_matches", {"name_to_match": "we're all wired"}]
#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct StepMatchesRequest {
  pub name_to_match: String
}

// ["invoke", {"id":"1", "args": []}]
// ["invoke", {"id":"1", "args": ["wired"]}]
// ["invoke", {"id":"1", "args": ["we're",[["wired"],["high"],["happy"]]]}]
// TODO: This requires manual encoding because of table being embedded
#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct InvokeRequest {
  pub id: String,
  pub args: Vec<InvokeArgument>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum InvokeArgument {
  String(String),
  Boolean(bool),
  Table(Vec<Vec<String>>)
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
    Ok(InvokeArgument::String(v.to_owned()))
  }

  fn visit_bool<E: SerdeError>(&mut self, _v: bool) -> Result<InvokeArgument, E> {
    Ok(InvokeArgument::Boolean(_v))
  }

  fn visit_seq<V: SeqVisitor>(&mut self, _visitor: V) -> Result<InvokeArgument, V::Error> {
    VecVisitor::new().visit_seq(_visitor).map(|res| InvokeArgument::Table(res))
  }
}

// ["begin_scenario"]
// ["begin_scenario", {"tags":["bar","baz","foo"]}]
#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct BeginScenarioRequest {
  pub tags: Vec<String>
}


// ["end_scenario"]]
// ["end_scenario", {"tags":["bar","baz","foo"]}]
#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct EndScenarioRequest {
  pub tags: Vec<String>
}

// ["snippet_text", {"step_keyword": "Given", "multiline_arg_class":"", "step_name":"we're all wired"}]
#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct SnippetTextRequest {
  pub step_keyword: String,
  pub multiline_arg_class: String,
  pub step_name: String
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
        assert_eq!(payload, InvokeRequest {id: "1".to_owned(), args: vec!(InvokeArgument::String("wired".to_owned()))})
      },
      _ => panic!("result was not Invoke type")
    }
  }

  #[test]
  fn read_invoke_bool() {
    let json = "[\"invoke\", {\"id\":\"1\", \"args\": [true]}]";
    let res = serde_json::from_str(json);
    println!("{:?}", res);
    match res.unwrap() {
      Request::Invoke(payload) => {
        assert_eq!(payload, InvokeRequest {id: "1".to_owned(), args: vec!(InvokeArgument::Boolean(true))})
      },
      _ => panic!("result was not Invoke type")
    }
  }

  #[test]
  fn read_invoke_complicated_args() {
    let json = "[\"invoke\", {\"id\":\"1\", \"args\": [\"we're\", false, [[\"wired\"],[\"high\"],[\"happy\"]]]}]";
    let res = serde_json::from_str(json);
    println!("{:?}", res);
    match res.unwrap() {
      Request::Invoke(payload) => {
        assert_eq!(payload, InvokeRequest {id: "1".to_owned(), args: vec!(
              InvokeArgument::String("we're".to_owned()),
              InvokeArgument::Boolean(false),
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
