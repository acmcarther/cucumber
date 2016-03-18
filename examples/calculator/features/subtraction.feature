Feature: Subtraction

  Background:
    Given a clear calculator

  Scenario: Subtracting two positive numbers
    When I begin subtracting
    And I input 5
    And I input 4
    Then the result is 1

  Scenario: Subtracting three numbers
    When I begin subtracting
    And I input 10
    And I input 7
    And I input -3
    Then the result is 6

  Scenario: Subtracting numbers resulting in a negative
    When I begin subtracting
    And I input 6
    And I input 10
    Then the result is -4
