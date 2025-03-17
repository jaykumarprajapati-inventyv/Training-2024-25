
const { defineConfig, devices } = require('@playwright/test');

module.exports = defineConfig({
  projects: [
    {
      name: 'playwright tests',
      use: { ...devices['Desktop Chrome'] },
      testDir: './features', 
    },
  ],
  reporter: [
    ['html', { outputFolder: 'reports/playwright-report', open: 'never' }], 
  ],
  timeout: 30000, 
});
