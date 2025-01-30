const { Given, When, Then } = require("@cucumber/cucumber");
const { chromium } = require("playwright");

let browser, page;

Given("I open the login page", async () => {
  browser = await chromium.launch({ headless: false });
  page = await browser.newPage();
  await page.goto("http://localhost:3000");
});

When("I enter {string} as username", async (username) => {
  await page.fill("#username", username);
});

When("I enter {string} as password", async (password) => {
  await page.fill("#password", password);
});

When("I click the {string} button", async (buttonText) => {
  await page.click("#loginButton");
});

Then("I should see {string}", async (expectedText) => {
  const actualText = await page.locator("h1").innerText();
  if (actualText !== expectedText) {
    throw new Error(`Expected: "${expectedText}", but got: "${actualText}"`);
  }
  await browser.close();
});
