const mongoose = require("mongoose");
require("dotenv").config();

const uri = process.env.MONGO_URI;

mongoose
  .connect(process.env.MONGO_URI, {
    // useNewUrlParser: true,
    // useUnifiedTopology: true,
    serverSelectionTimeoutMS: 30000, // Increase timeout (30 seconds)
  })
  .then(() => console.log("MongoDB connected"))
  .catch((err) => console.log(err));
