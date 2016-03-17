Feature: The calculator buttons work

  Background: The calculator is clear
    Given the calculator is clear

  Scenario: I push some buttons
    When the number 5 is entered
    Then the display says 5

  Scenario: I push some other buttons
    When the number 1 is entered
    Then the display says 1
    Then the current tag state contains the tag "lol"
