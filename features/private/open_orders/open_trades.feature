Feature: Open Trades endpoint

  Scenario: User want to know all his open trades
    Given User with correct api keys
    And without any open trades
    When query for open trades
    Then list is empty
