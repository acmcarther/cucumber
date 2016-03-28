Feature: Calling steps within steps

  Background:
    Given a project if I don't already have one
    And the steps
      """
        When!(c, "^I get invoked indirectly$", |_, _, _| {
          InvokeResponse::fail_from_str("Indirect step got invoked!")
        });

        When!(c, "^I invoke a step indirectly$", |cuke: &Cucumber<u32>, world: &mut u32, _| {
          cuke.invoke("I get invoked indirectly", world, None)
        });
      """
    Then the project compiles

  Scenario: Invoking a step indirectly
    When the following feature is executed
      """
        Feature: Invoking step indirectly
          Scenario: Invoking
            When I invoke a step indirectly
      """
    Then the feature fails with "Indirect step got invoked!"
