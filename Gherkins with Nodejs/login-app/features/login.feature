Feature: Login Button Automation

  Scenario Outline: Attempt login with different credentials
    Given I open the login page
    When I enter "<username>" as username
    And I enter "<password>" as password
    And I click the "Login" button
    Then I should see "<expected_result>"

  Examples:
    | username | password | expected_result              |
    | jay      | 1234     | Successfully logged in       |
    | umang    | 1234     | Incorrect credentials!!      |
    | jay      | 0000     | Incorrect credentials!!      |
    | test     | wrong    | Incorrect credentials!!      |
