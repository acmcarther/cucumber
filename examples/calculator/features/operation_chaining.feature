Feature: Operation chaining

  Background:
    Given a clear calculator

  Scenario: Chaining several operations
    When I begin adding
    And I input 5
    And I input 5
    And I begin subtracting
    And I input 4
    And I begin adding
    And I input 6
    Then the result is 12
