Feature: Explicit (early) success
  Background:
    Given a project if I don't already have one
    And the steps
      """
        Given!(c, "^an ordinary step$", |c: &Cucumber<u32>, _, _| {
          c.succeed_immediately();
          panic!("Unreachable panic!");
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
    Then the feature passes with no undefined steps
