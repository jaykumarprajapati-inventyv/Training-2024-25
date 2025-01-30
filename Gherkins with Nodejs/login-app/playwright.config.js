module.exports = {
    timeout: 30000,
    use: {
        headless: false, 
    },
    reporter: [['html', { outputFolder: 'test-results', open: 'never' }]],
};
