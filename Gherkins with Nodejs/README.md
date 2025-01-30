# Cucumber with Node js

## Overview
This project is an automated testing setup using Gherkin with Node.js and Playwright for end-to-end testing of a login application. It follows a structured approach where Gherkin feature files define test scenarios in the features/ folder, and step definitions implement them. The tests/ directory contains Playwright test scripts, while test-results/ stores execution reports. The project is managed with Node.js, with dependencies handled via package.json. A server.js file is included, possibly for running a local server. Configuration files like playwright.config.js ensure smooth test execution, and documentation is available in README.md. 🚀

## Folder Structure

This is the folder structure:

```
Gherkins with Nodejs/             # Root folder
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
