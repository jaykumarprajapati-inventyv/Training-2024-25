require("dotenv").config();
const express = require("express");
const axios = require("axios");
const cors = require("cors");

const app = express();
app.use(express.json());

app.use(cors({ origin: "*" }));

const API_KEY = process.env.OPENROUTER_API_KEY;
const API_URL = "https://openrouter.ai/api/v1/chat/completions";

app.post("/chat", async (req, res) => {
  try {

    let { messages } = req.body;

    if (!Array.isArray(messages) || messages.length === 0) {
      return res.status(400).json({ error: "No valid messages to process." });
    }
    messages = messages
      .map((msg) => ({
        role: msg.role === "bot" ? "assistant" : msg.role,
        content: msg.content?.trim() || null,
      }))
      .filter((msg) => msg.content);

    if (messages.length === 0) {
      return res
        .status(400)
        .json({ error: "No valid messages after processing." });
    }


    const response = await axios.post(
      API_URL,
      { model: "gpt-3.5-turbo", messages },
      {
        headers: {
          Authorization: `Bearer ${API_KEY}`,
          "Content-Type": "application/json",
        },
      }
    );

  
    res.json(response.data);
  } catch (error) {
    console.error(
      "API Request Failed:",
      error.response
        ? JSON.stringify(error.response.data, null, 2)
        : error.message
    );
    res.status(500).json({
      error: "API Request Failed",
      details: error.response?.data || error.message,
    });
  }
});

app.listen(3000, () => console.log("My Server is running on port 3000"));
