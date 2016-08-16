Feature: Pending Steps

  Background:
    Given a project if I don't already have one
    And the steps
      """
      Given!(c, "^a pending step$", |c: &Cucumber<u32>, _, _| {
          c.pending("Test step is pending");
        });
      """
    Then the project compiles

  Scenario: Executing a pending step
    When the following feature is executed
      """
        Feature: I do a thing
          Scenario: The thing works
            Given a pending step
      """
    Then the feature passes
    And the feature reports a pending step

