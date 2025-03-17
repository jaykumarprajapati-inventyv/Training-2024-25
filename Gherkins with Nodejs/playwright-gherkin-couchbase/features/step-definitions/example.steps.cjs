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

   
    await this.page.setDefaultTimeout(50000); 
  }

  async close() {
    
    await this.browser.close();
  }
}


setWorldConstructor(CustomWorld);


Before(async function () {
  await this.init(); 
});


After(async function () {
  await this.close(); 
});


Given("I open the website {string}", async function (url) {
  await this.page.goto(url); 
});

When("I add the first item to the cart", async function () {
  await this.page
    .locator("div:nth-child(3) > .pricebar > .btn_primary")
    .click(); 
});

When("I add the second item priced at $29.99 to the cart", async function () {
  await this.page
    .locator("div")
    .filter({ hasText: /^\$29\.99ADD TO CART$/ })
    .getByRole("button")
    .click(); 
});

When("I add the third item priced at $49.99 to the cart", async function () {
  await this.page
    .locator("div")
    .filter({ hasText: /^\$49\.99ADD TO CART$/ })
    .getByRole("button")
    .click(); 
});

When("I navigate to the cart", async function () {
  await this.page.getByRole("link", { name: "3" }).click(); 
});

When("I proceed to checkout", async function () {
  await this.page.getByRole("link", { name: "CHECKOUT" }).click(); 
});

When("I enter first name {string}", async function (firstName) {
  await this.page.locator('[data-test="firstName"]').fill(firstName); 
});

When("I enter last name {string}", async function (lastName) {
  await this.page.locator('[data-test="lastName"]').fill(lastName); 
});

When("I enter postal code {string}", async function (postalCode) {
  await this.page.locator('[data-test="postalCode"]').fill(postalCode); 
});

When("I continue to the next step", async function () {
  await this.page.getByRole("button", { name: "CONTINUE" }).click(); 
});

Then("I should be able to finish the purchase", async function () {
  await this.page.getByRole("link", { name: "FINISH" }).click(); 
});
