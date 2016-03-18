Feature: Evaluation

  Background:
    Given a clear calculator

  Scenario: Evaluating an empty expression
    Then the result is 0

  Scenario: Evaluating an expression clears the buffer
    When I begin adding
    And I input 5
    And I input 4
    Then the result is 9
    And the next result is 0
