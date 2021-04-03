Feature: Time feature

  Scenario: User query for XBTUSD pair using public api
    Given User without credentials
    When query for ticker pair "XBTUSD"
    Then field "altname" in response contains "XBTUSD"
    And field "wsname" in response contains "XBT/USD"
    And field "aclass_base" in response contains "currency"
    And field "base" in response contains "XXBT"
    And field "aclass_quote" in response contains "currency"
    And field "quote" in response contains "ZUSD"
    And field "lot" in response contains "unit"
    And field "pair_decimals" in response contains "1"
    And field "lot_decimals" in response contains "8"
    And field "lot_multiplier" in response contains "1"
    And field "leverage_buy" in response contains "[Number(2), Number(3), Number(4), Number(5)]"
    And field "ordermin" in response contains "0.0002"

  Scenario: User query for non existing pair using public api
    Given User without credentials
    When query for ticker pair "XBTUSDN"
    Then error is produced with messsage ~ "Unknown asset pair"
