Feature: Error Handling

  Background:
    Given a clear calculator

  Scenario: Inputting numbers before an operator
    When I input 5
    Then the last message includes "Input operator first"
