const { test, expect } = require("@playwright/test");

const loginCases = [
  {
    username: "jay",
    password: "1234",
    expectedURL: "http://localhost:3000/success",
    expectedText: "Successfully logged in",
  },
  {
    username: "umang",
    password: "1234",
    expectedURL: "http://localhost:3000/failure",
    expectedText: "Incorrect credentials!!",
  },
  {
    username: "jay",
    password: "0000",
    expectedURL: "http://localhost:3000/failure",
    expectedText: "Incorrect credentials!!",
  },
  {
    username: "test",
    password: "wrong",
    expectedURL: "http://localhost:3000/failure",
    expectedText: "Incorrect credentials!!",
  },
];

loginCases.forEach(({ username, password, expectedURL, expectedText }) => {
  test(`Login with username "${username}" and password "${password}"`, async ({
    page,
  }) => {
    await page.goto("http://localhost:3000");
    await page.fill("#username", username);
    await page.fill("#password", password);
    await page.click("#loginButton");

    await expect(page).toHaveURL(expectedURL);
    await expect(page.locator("h1")).toHaveText(expectedText);

    console.log(
      `Test completed for username: ${username}, Expected: ${expectedText}`
    );
  });
});
