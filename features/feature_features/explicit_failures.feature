Feature: Explicit failures
  Background:
    Given a project if I don't already have one
    And the steps
      """
        Given!(c, "^an ordinary step$", |c: &Cucumber<u32>, _, ()| {
          c.fail("I just don't want this step to pass");
        });
      """
    Then the project compiles

  Scenario: Evaluating step that explicitly fails
    When the following feature is executed
      """
        Feature: I do a thing
          Scenario: The thing works
            Given an ordinary step
      """
    Then the feature fails with "I just don't want this step to pass"

