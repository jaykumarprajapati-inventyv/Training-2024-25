const mongoose = require("mongoose");
require("dotenv").config();

const uri = process.env.MONGO_URI;

mongoose
  .connect(process.env.MONGO_URI, {
   
    serverSelectionTimeoutMS: 30000, 
  })
  .then(() => console.log("MongoDB connected"))
  .catch((err) => console.log(err));
