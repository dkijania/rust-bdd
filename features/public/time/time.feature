Feature: Time endpoint

  Scenario: User wants to know current server time
    Given User without api keys
    When query for server time
    Then date close to current time is produced
    And human readable date has "%a,  %e %b %y %T %z" format
