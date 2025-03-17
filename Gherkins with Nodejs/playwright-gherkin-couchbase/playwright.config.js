
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  reporter: [
    ['json', { outputFile: 'results.json' }]  // This will generate the results in `results.json`
  ],
  use: {
    // Device configurations for testing, adjust as needed
    ...devices['Desktop Chrome'],
  },
});
