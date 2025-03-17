Feature: Purchase flow on Saucedemo

  Scenario: Successful purchase on Saucedemo
   Given I open the website "https://www.saucedemo.com/v1/inventory.html"
    When I add the first item to the cart
    And I add the second item priced at $29.99 to the cart
    And I add the third item priced at $49.99 to the cart
    And I navigate to the cart
    And I proceed to checkout
    And I enter first name "Anil"
    And I enter last name "Lamba"
    And I enter postal code "598765"
    And I continue to the next step
    Then I should be able to finish the purchase
