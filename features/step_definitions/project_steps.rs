use cucumber::{
  Cucumber,
  CucumberRegistrar,
  InvokeResponse
};
use support::env::CucumberWorld;
use support::fs;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CucumberWorld>) {
  Given!(c, "^a project$", |_, world: &mut CucumberWorld, _| {
    match fs::create_project() {
      Ok(current_project) => {
        world.current_project = Some(current_project);
        InvokeResponse::Success
      },
      Err(ref err) => InvokeResponse::fail_from_str(&format!("Failed to create project {}", err))
    }
  });

  Given!(c, "^a project if I don't already have one$", |cuke: &Cucumber<CucumberWorld>, world: &mut CucumberWorld, _| {
    match world.current_project {
      Some(_) => InvokeResponse::Success,
      None => cuke.invoke("a project", world, None)
    }
  });

  Given!(c, "^the steps$", |_, world: &mut CucumberWorld, (step_set,): (String,)| {
    match world.current_project {
      None => return InvokeResponse::fail_from_str("There was no project to add steps to"),
      Some(ref mut project) => {
        project.set_steps(&step_set);
        InvokeResponse::Success
      }
    }
  });

  Then!(c, "^the project compiles$", |_, world: &mut CucumberWorld, _| {
    match world.current_project {
      None => return InvokeResponse::fail_from_str("There was no project to compile"),
      Some(ref mut project) => {
        match project.compile() {
          Err(err) => InvokeResponse::fail_from_str(&format!("The project failed to compile with: {}", err)),
          Ok(_) => InvokeResponse::Success
        }
      }
    }
  });

  When!(c, "^the following feature is executed$", |_, world: &mut CucumberWorld, (scenario,): (String,)| {
    match world.current_project {
      None => return InvokeResponse::fail_from_str("There was no project to compile"),
      Some(ref mut project) => {
        let result = project.execute_feature(&scenario);
        world.execute_result = Some(result);
        InvokeResponse::Success
      }
    }

  });

  Then!(c, "^the feature passes with no undefined steps$", |cuke: &Cucumber<CucumberWorld>, world: &mut CucumberWorld, _| {
    cuke.invoke("the feature passes", world, None)
      .and(cuke.invoke("the feature reports no undefined steps", world, None))
  });

  Then!(c, "^the feature passes$", |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => InvokeResponse::fail_from_str("Expected there to be an execute result but there wasn't one"),
      Some(Err(ref err)) => InvokeResponse::fail_from_str(&format!("Expected scenario to pass but it failed with {}", err)),
      _ => InvokeResponse::Success
    }
  });

  Then!(c, "^the feature fails with \"([^\"]*)\"$", |_, world: &mut CucumberWorld, (fail_msg,): (String,)| {
    match world.execute_result {
      None => InvokeResponse::fail_from_str("Expected there to be an execute result but there wasn't one"),
      Some(Ok(_)) => InvokeResponse::fail_from_str("Expected scenario to fail but it passed"),
      Some(Err(ref err)) => {
        InvokeResponse::expect(err.contains(&fail_msg), &format!("expected to find {} in error message: {}", fail_msg, err))
      }
    }
  });

  Then!(c, "^the feature reports an undefined step$", |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => InvokeResponse::fail_from_str("Expected there to be an execute result but there wasn't one"),
      Some(Err(_)) => InvokeResponse::fail_from_str("Expected scenario to pass (to retrieve an undefined step) but it failed"),
      Some(Ok(ref output)) => InvokeResponse::expect(output.contains("1 undefined"), "Expected scenario output to contain exactly one undefined step")
    }
  });

  Then!(c, "^the feature reports no undefined steps$", |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => InvokeResponse::fail_from_str("Expected there to be an execute result but there wasn't one"),
      Some(Err(_)) => InvokeResponse::fail_from_str("Expected scenario to pass (to retrieve an undefined step) but it failed"),
      Some(Ok(ref output)) => InvokeResponse::expect(!output.contains(" undefined"), &format!("Expected scenario output to contain no undefined steps, but it contained some: {}", output))
    }
  });

  Then!(c, "^the feature reports a pending step$", |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => InvokeResponse::fail_from_str("Expected there to be an execute result but there wasn't one"),
      Some(Err(_)) => InvokeResponse::fail_from_str("Expected scenario to pass (to retrieve a pending step) but it failed"),
      Some(Ok(ref output)) => InvokeResponse::expect(output.contains("Test step is pending"), "Expected scenario output to contain pending step")
    }
  });
}
