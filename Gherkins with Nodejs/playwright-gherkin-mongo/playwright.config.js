// const { defineConfig, devices } = require('@playwright/test');

// module.exports = defineConfig({
//   projects: [
//     {
//       name: 'playwright tests',
//       use: { ...devices['Desktop Chrome'] }, 
//       testDir: './features',
//     },
//   ],
//   reporter: [
//     ['html', { outputFolder: 'reports/playwright-report', open: 'never' }],
//   ],
//   timeout: 30000, 
// });
// playwright.config.js
const { defineConfig, devices } = require('@playwright/test');

module.exports = defineConfig({
  projects: [
    {
      name: 'playwright tests',
      use: { ...devices['Desktop Chrome'] },
      testDir: './features', // Your test directory containing feature files
    },
  ],
  reporter: [
    ['html', { outputFolder: 'reports/playwright-report', open: 'never' }], // HTML report generation
  ],
  timeout: 30000, // Adjust timeout if needed
});
