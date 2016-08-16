use cucumber::{Cucumber, CucumberRegistrar, InvokeArgument};
use support::env::CucumberWorld;
use support::fs;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CucumberWorld>) {
  Given!(c, "^a project$", |_, world: &mut CucumberWorld, _| {
    match fs::create_project() {
      Ok(current_project) => {
        world.current_project = Some(current_project);
      },
      Err(ref err) => panic!("Failed to create project {}", err),
    }
  });

  Given!(c,
         "^a project if I don't already have one$",
         |cuke: &Cucumber<CucumberWorld>, world: &mut CucumberWorld, _| {
           match world.current_project {
             None => cuke.invoke("a project", world, None),
             _ => {},
           }
         });

  Given!(c,
         "^the steps$",
         |_, world: &mut CucumberWorld, (step_set,): (String,)| {
    match world.current_project {
      None => return panic!("There was no project to add steps to"),
      Some(ref mut project) => {
        project.set_steps(&step_set);
      },
    }
  });

  Then!(c,
        "^the project compiles$",
        |_, world: &mut CucumberWorld, _| {
    match world.current_project {
      None => return panic!("There was no project to compile"),
      Some(ref mut project) => {
        match project.compile() {
          Err(err) => panic!("The project failed to compile with: {}", err),
          _ => {},
        }
      },
    }
  });

  When!(c,
        "^the following feature is executed$",
        |c: &Cucumber<CucumberWorld>, world: &mut CucumberWorld, (scenario,): (String,)| {
          c.invoke("the following feature is executed with \"\"",
                   world,
                   Some(InvokeArgument::String(scenario)));
        });

  When!(c,
        "^the following feature is executed with \"(.*)\"$",
        |_, world: &mut CucumberWorld, (args, scenario): (String, String)| {
    match world.current_project {
      None => return panic!("There was no project to compile"),
      Some(ref mut project) => {
        let result = project.execute_feature(&scenario, &args);
        world.execute_result = Some(result);
      },
    }

  });


  Then!(c,
        "^the feature passes with no undefined steps$",
        |cuke: &Cucumber<CucumberWorld>, world: &mut CucumberWorld, _| {
          cuke.invoke("the feature passes", world, None);
          cuke.invoke("the feature reports no undefined steps", world, None);
        });

  Then!(c,
        "^the feature passes$",
        |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => panic!("Expected there to be an execute result but there wasn't one"),
      Some(Err(ref err)) => panic!("Expected scenario to pass but it failed with {}", err),
      _ => {},
    }
  });

  Then!(c,
        "^the feature fails with \"([^\"]*)\"$",
        |_, world: &mut CucumberWorld, (fail_msg,): (String,)| {
    match world.execute_result {
      None => panic!("Expected there to be an execute result but there wasn't one"),
      Some(Ok(_)) => panic!("Expected scenario to fail but it passed"),
      Some(Err(ref err)) => assert!(err.contains(&fail_msg)),
    }
  });

  Then!(c,
        "^the feature reports an undefined step$",
        |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => panic!("Expected there to be an execute result but there wasn't one"),
      Some(Err(_)) => {
        panic!("Expected scenario to pass (to retrieve an undefined step) but it failed")
      },
      Some(Ok(ref output)) => assert!(output.contains("1 undefined")),
    }
  });

  Then!(c,
        "^the feature reports no undefined steps$",
        |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => panic!("Expected there to be an execute result but there wasn't one"),
      Some(Err(_)) => {
        panic!("Expected scenario to pass (to retrieve an undefined step) but it failed")
      },
      Some(Ok(ref output)) => assert!(!output.contains(" undefined")),
    }
  });

  Then!(c,
        "^the feature reports a pending step$",
        |_, world: &mut CucumberWorld, _| {
    match world.execute_result {
      None => panic!("Expected there to be an execute result but there wasn't one"),
      Some(Err(_)) => {
        panic!("Expected scenario to pass (to retrieve a pending step) but it failed")
      },
      Some(Ok(ref output)) => assert!(output.contains("Test step is pending")),
    }
  });
}
