Feature: Open Trades feature

  Scenario: User query his open trades
    Given User with correct api keys
    And Does not have any open trades
    When User query for open trades
    Then open trades are empty
