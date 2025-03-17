const {
  Given,
  When,
  Then,
  Before,
  After,
  setWorldConstructor,
} = require("@cucumber/cucumber");
const { chromium } = require("@playwright/test");

class CustomWorld {
  constructor() {
    this.browser = null;
    this.page = null;
  }

  async init() {
    
    this.browser = await chromium.launch({
      headless: false, 
      slowMo: 50, 
    });
    this.page = await this.browser.newPage();

   
    await this.page.setDefaultTimeout(50000); // Increase default timeout to 30 seconds
  }

  async close() {
    // Close the browser after the scenario
    await this.browser.close();
  }
}

// Set the world constructor to the custom world object
setWorldConstructor(CustomWorld);

// Before hook to initialize browser and page
Before(async function () {
  await this.init(); // Initialize Playwright browser and page before each scenario
});

// After hook to close the browser
After(async function () {
  await this.close(); // Close browser after each scenario
});

// Implementing the steps for the scenario
Given("I open the website {string}", async function (url) {
  await this.page.goto(url); // Navigate to the given URL
});

When("I add the first item to the cart", async function () {
  await this.page
    .locator("div:nth-child(3) > .pricebar > .btn_primary")
    .click(); // Click on the first item to add it to the cart
});

When("I add the second item priced at $29.99 to the cart", async function () {
  await this.page
    .locator("div")
    .filter({ hasText: /^\$29\.99ADD TO CART$/ })
    .getByRole("button")
    .click(); // Add second item to the cart
});

When("I add the third item priced at $49.99 to the cart", async function () {
  await this.page
    .locator("div")
    .filter({ hasText: /^\$49\.99ADD TO CART$/ })
    .getByRole("button")
    .click(); // Add third item to the cart
});

When("I navigate to the cart", async function () {
  await this.page.getByRole("link", { name: "3" }).click(); // Navigate to the cart (with 3 items in it)
});

When("I proceed to checkout", async function () {
  await this.page.getByRole("link", { name: "CHECKOUT" }).click(); // Proceed to checkout
});

When("I enter first name {string}", async function (firstName) {
  await this.page.locator('[data-test="firstName"]').fill(firstName); // Fill first name
});

When("I enter last name {string}", async function (lastName) {
  await this.page.locator('[data-test="lastName"]').fill(lastName); // Fill last name
});

When("I enter postal code {string}", async function (postalCode) {
  await this.page.locator('[data-test="postalCode"]').fill(postalCode); // Fill postal code
});

When("I continue to the next step", async function () {
  await this.page.getByRole("button", { name: "CONTINUE" }).click(); // Click continue button to proceed
});

Then("I should be able to finish the purchase", async function () {
  await this.page.getByRole("link", { name: "FINISH" }).click(); // Finish the purchase by clicking the finish button
});
