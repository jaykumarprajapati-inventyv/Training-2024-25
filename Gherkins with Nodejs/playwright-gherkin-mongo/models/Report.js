const mongoose = require('mongoose');

const reportSchema = new mongoose.Schema({
  reportData: Object,  // To store the JSON report
  createdAt: { type: Date, default: Date.now },  // To store the creation time
});

module.exports = mongoose.model('Report', reportSchema);
