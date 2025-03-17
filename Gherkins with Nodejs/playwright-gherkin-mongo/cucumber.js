module.exports = {
  default: {
    require: ["features/step-definitions/*.js"],
    format: ["json:test-results.json", "html:test-results.html"],
    timeout: 60000, 
  },
};
