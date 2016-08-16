pub struct CucumberWorld {
  pub current_project: Option<super::fs::Project>,
  pub execute_result: Option<Result<String, String>>,
}

impl CucumberWorld {
  pub fn new() -> CucumberWorld {
    CucumberWorld {
      current_project: None,
      execute_result: None,
    }
  }
}
