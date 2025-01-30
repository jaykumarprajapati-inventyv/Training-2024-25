Feature: Login Button Automation

  Scenario: Click the login button after entering right credentials
    Given I open the login page
    When I enter "jay" as username
    And I enter "1234" as password
    And I click the "Login" button
    Then I should see "Successfully logged in"

  Scenario: Click the login button after entering wrong credentials
   Given I open the login page
   When I enter "raj" as username
   When I enter "4567" as password
   And I click the "Login" button
   Then I should see "Incorrect credentials!!"
