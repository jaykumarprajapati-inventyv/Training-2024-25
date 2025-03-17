const { Given, When, Then, setDefaultTimeout } = require('@cucumber/cucumber');
const { chromium } = require('playwright');

let browser, page;


setDefaultTimeout(60000); 

Given('I open the SauceDemo inventory page', async function () {
  browser = await chromium.launch({ headless: false, slowMo: 500 });
  page = await browser.newPage();
  
  
  await page.goto('https://www.saucedemo.com/v1/inventory.html', { waitUntil: 'load' });
  await page.waitForLoadState('networkidle'); 
});

When('I add items to the cart', async function () {
  await page.locator('div').filter({ hasText: /^\$29\.99ADD TO CART$/ }).getByRole('button').click();
  await page.locator('div').filter({ hasText: /^\$49\.99ADD TO CART$/ }).getByRole('button').click();
  await page.locator('div').filter({ hasText: /^\$7\.99ADD TO CART$/ }).getByRole('button').click();
});

When('I proceed to checkout', async function () {
  await page.getByRole('link', { name: '3' }).click();
  await page.getByRole('link', { name: 'CHECKOUT' }).click();
});

When('I fill in the user details', async function () {
  await page.locator('[data-test="firstName"]').click();
  await page.locator('[data-test="firstName"]').fill('Sanjay');
  await page.locator('[data-test="lastName"]').click();
  await page.locator('[data-test="lastName"]').fill('Lamba');
  await page.locator('[data-test="postalCode"]').click();
  await page.locator('[data-test="postalCode"]').fill('384561');
  await page.getByRole('button', { name: 'CONTINUE' }).click();
});

Then('I complete the purchase', async function () {
  
  await page.waitForLoadState('networkidle');

  
  const finishButton = page.getByRole('button', { name: 'FINISH' });

  await finishButton.waitFor({ state: 'visible', timeout: 30000 }); 
  await finishButton.click();

  console.log("Purchase completed successfully!");

  await browser.close();
});




// features/step-definitions/inventory.js
// const { Given, When, Then } = require('@cucumber/cucumber');
// const { chromium } = require('playwright');

// let browser, page;

// Given('I open the SauceDemo inventory page', async function () {
//   browser = await chromium.launch({ headless: false, slowMo: 500 });
//   page = await browser.newPage();
//   await page.goto('https://www.saucedemo.com/v1/inventory.html');
// });

// When('I add items to the cart', async function () {
//   await page.locator('div').filter({ hasText: /^\$29\.99ADD TO CART$/ }).getByRole('button').click();
//   await page.locator('div').filter({ hasText: /^\$49\.99ADD TO CART$/ }).getByRole('button').click();
//   await page.locator('div').filter({ hasText: /^\$7\.99ADD TO CART$/ }).getByRole('button').click();
// });

// When('I proceed to checkout', async function () {
//   await page.getByRole('link', { name: '3' }).click();
//   await page.getByRole('link', { name: 'CHECKOUT' }).click();
// });

// When('I fill in the user details', async function () {
//   await page.locator('[data-test="firstName"]').click();
//   await page.locator('[data-test="firstName"]').fill('Sanjay');
//   await page.locator('[data-test="lastName"]').click();
//   await page.locator('[data-test="lastName"]').fill('Lamba');
//   await page.locator('[data-test="postalCode"]').click();
//   await page.locator('[data-test="postalCode"]').fill('384561');
//   await page.getByRole('button', { name: 'CONTINUE' }).click();
// });

// Then('I complete the purchase', async function () {
//   await page.getByRole('link', { name: 'FINISH' }).click();
//   await browser.close();
// });


