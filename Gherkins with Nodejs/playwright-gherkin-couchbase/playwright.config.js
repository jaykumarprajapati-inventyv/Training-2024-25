
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  reporter: [
    ['json', { outputFile: 'results.json' }]  
  ],
  use: {
    
    ...devices['Desktop Chrome'],
  },
});
