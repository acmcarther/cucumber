use regex::Regex;

#[allow(dead_code)]
pub fn r(str: &'static str) -> Regex {
  Regex::new(str).unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_makes_a_regex() {
    let regex = r("^Hello Regex$");
    assert!(regex.is_match("Hello Regex"));
  }
}
