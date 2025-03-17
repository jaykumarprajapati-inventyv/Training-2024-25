const express = require("express");
const cors = require("cors");
const mongoose = require("mongoose");
const Report = require("./models/Report.js");
require("dotenv").config();

const path = require("path");
const app = express();
app.use(express.json());
app.use(cors());

app.use(express.static(path.join(__dirname, "public")));
mongoose
  .connect(process.env.MONGO_URI, {
    useNewUrlParser: true,
    useUnifiedTopology: true,
  })
  .then(() => console.log("MongoDB connected"))
  .catch((err) => console.log(err));

// API endpoint to fetch the latest report
app.get("/get-report", async (req, res) => {
  try {
    const report = await Report.find().sort({ createdAt: -1 }).limit(1); // Fetch the latest report
    res.json(report[0].reportData); // Send the report data as JSON
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// API endpoint to fetch the report and render it as HTML
app.get("/report", async (req, res) => {
  try {
    const report = await Report.find().sort({ createdAt: -1 }).limit(1);
    const reportData = report[0].reportData;

    // Render the report in HTML format (simplified)
    const htmlReport = `
      <html>
        <head><title>Test Report</title></head>
        <body>
          <h1>Test Report</h1>
          <pre>${JSON.stringify(reportData, null, 2)}</pre>
        </body>
      </html>
    `;
    res.send(htmlReport); // Send HTML response
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Start the server
app.listen(3001, () => {
  console.log("Server running on port 3001");
});
