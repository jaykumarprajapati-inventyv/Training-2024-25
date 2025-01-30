# Task1: Automation Testing Project 

## About the Project. :writing_hand:
The Automation Testing Project is made by **Node.js** for server, **Playwright** and **Cucumber.js** for end-to-end testing, it focused on login functionality. It generates report system to know how many test cases passed or failed out of total cases.


## 🛠️ Prerequisites
- Node.js v16+ ([Download](https://nodejs.org/))
- npm (comes with Node.js)
- Modern browser (Chromium, Firefox, or WebKit)


## 📂 Project Structure

```
Gherkins with Nodejs/       # Root folder
│
├── login-app/               # Main project folder
│   ├── features/            # Contains Gherkin feature files and step definitions
│   │   ├── login.feature    # Feature file for login testing
│   │   ├── step-definitions/  # Folder for step definitions
│   │
│   ├── node_modules/        # Dependencies (not expanded)
│   │    ├── .../              #Contains all dependencies
│   │
│   ├── test-results/        # Stores test run results
│   │   ├── .last-run.json   # JSON file with last test run details
│   │   ├── index.html       # Test report in HTML format
│   │
│   ├── tests/               # Contains test files
│   │   ├── login.spec.js    # Playwright test file
│   │
│   ├── package-lock.json    # Auto-generated file for package dependencies
│   ├── package.json         # Project configuration file
│   ├── playwright.config.js # Playwright configuration file
│   ├── README.md            # Documentation file
│   ├── server.js            # Backend server script
│
└── README.md                # General documentation for the project

```


# 🚀 How to Run Tests

## First Start a Local Server
```bash
node server.js
```

## Run Cucumber Tests 
```bash
npx cucumber-js
```

## Run Playwright Tests + Generate Playwright Report
```bash
npx playwright test
```