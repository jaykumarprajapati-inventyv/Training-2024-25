const mongoose = require("mongoose");
const fs = require("fs");
const Report = require("./models/Report");x``
require("dotenv").config();

mongoose
  .connect(process.env.MONGO_URI, {
    useNewUrlParser: true,
    useUnifiedTopology: true,
  })
  .then(() => console.log("MongoDB connected"))
  .catch((err) => console.log("MongoDB connection error:", err));

const reportJson = JSON.parse(fs.readFileSync("test-results.json", "utf-8"));

const saveReport = async () => {
  try {
    await Report.create({ reportData: reportJson });
    console.log("Report saved to MongoDB");
  } catch (err) {
    console.error(err);
  }
};

saveReport();
