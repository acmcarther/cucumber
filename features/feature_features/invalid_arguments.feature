Feature: Invalid argument types

  Background:
    Given a project if I don't already have one
    And the steps
      """
        Given!(c, "^a step with non-matching args$", |_, _, (b,): (bool,)| {
          // Not reachable
          InvokeResponse::Success
        });

        Given!(c, "^another step with unparsable table arg$", |_, _, (b,): (bool,)| {
          // Not reachable
          InvokeResponse::Success
        });

        Given!(c, "^another step with unparseable args: \"(.*)\"$", |_, _, (u,): (u32,)| {
          // Not reachable
          InvokeResponse::Success
        });
      """
    Then the project compiles

  Scenario: Evaluating step with missing args
    When the following feature is executed
      """
        Feature: I do a thing
          Scenario: The thing works
            Given a step with non-matching args
      """
    Then the feature fails with "Expected [1] arguments, but found [0]"

  Scenario: Evaluating step with ill typed table args
    When the following feature is executed
      """
        Feature: I do a thing
          Scenario: The thing works
            Given another step with unparseable table arg
              | 1 | 2 | 3 |
      """
    Then the feature fails with "Argument in position [0] did not have the correct type or was unparseable"

  Scenario: Evaluating step with ill typed normal args
    When the following feature is executed
      """
        Feature: I do a thing
          Scenario: The thing works
            Given another step with unparseable args: "NAN"
      """
    Then the feature fails with "Argument in position [0] did not have the correct type or was unparseable"

