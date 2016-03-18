Feature: Registering cucumber steps

  Background:
    Given a new cucumber instance

  Scenario Outline: Adding a new step definition
    When I add a "<TYPE>" step definition with the regex "^example(?: step)?$"
    Then cucumber can find a step matching "example"
    And cucumber can find a step matching "example step"

    Examples:
      | TYPE  |
      | When  |
      | Given |
      | Then  |
