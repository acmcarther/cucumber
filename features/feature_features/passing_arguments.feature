Feature: Invalid argument types

  Background:
    Given a project if I don't already have one
    And the steps
      """
        Given!(c, "^I just explode$", |c: &Cucumber<u32>, _, _| {
          c.fail("Should not have invoked this scenario");
        });

        Given!(c, "^I run normally$", |_, _, _| {
        });
      """
    Then the project compiles

  Scenario: Evaluating step with missing args
    When the following feature is executed with "-t @RunOnly"
      """
        Feature: I do a thing
          Scenario: This will explode
            Given I just explode

          @RunOnly
          Scenario: I don't explode
            Given I run normally
      """
    Then the feature passes with no undefined steps
