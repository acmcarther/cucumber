Feature: Addition

  Background:
    Given a clear calculator

  Scenario: Adding two positive numbers
    When I begin adding
    And I input 5
    And I input 4
    Then the result is 9

  Scenario: Adding three numbers
    When I begin adding
    And I input 2
    And I input -7
    And I input 20
    Then the result is 15

  Scenario: Adding numbers resulting in a negative
    When I begin adding
    And I input 6
    And I input -10
    Then the result is -4

