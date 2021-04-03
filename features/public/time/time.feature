Feature: Time feature

  Scenario: User query for current time using public api
    Given User without credentials
    When query for time
    Then date close to current time is produced
    And human readable date has "%a,  %e %b %y %T %z" format
