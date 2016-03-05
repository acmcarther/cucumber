Feature: Cucumber Tags

  Scenario: Run with no tags
    When I run a scenario with no tags
    Then the current tag state contains no tags


  Scenario: Run with tags
    When I run a scenario with tag "@first-example"
    Then the current tag state contains the tag "@first-example"
