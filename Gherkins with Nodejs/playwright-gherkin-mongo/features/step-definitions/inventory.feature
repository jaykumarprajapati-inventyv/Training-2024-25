Feature: SauceDemo Shopping Cart

  Scenario: User adds items to the cart and completes checkout
    Given I open the SauceDemo inventory page
    When I add items to the cart
    And I proceed to checkout
    And I fill in the user details
    Then I complete the purchase
