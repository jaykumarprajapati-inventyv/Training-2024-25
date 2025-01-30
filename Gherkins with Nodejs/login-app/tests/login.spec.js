const { test, expect } = require("@playwright/test");

test("Login should be successful and go to success page", async ({ page }) => {
  await page.goto("http://localhost:3000");
  await page.fill("#username", "jay");
  await page.fill("#password", "1234");
  await page.click("#loginButton");

  await expect(page).toHaveURL("http://localhost:3000/success");
  await expect(page.locator("h1")).toHaveText("Successfully logged in");

  console.log("You've Login successfully!!");
});

test("Login should fail and go to failure page", async ({ page }) => {
    await page.goto("http://localhost:3000");
    await page.fill("#username", "raj");
    await page.fill("#password", "4567");
    await page.click("#loginButton");
  
   
    await expect(page).toHaveURL("http://localhost:3000/failure");
    await expect(page.locator("h1")).toHaveText("Incorrect credentials!!"); 
  
    console.log("Login failed!!");
  });
