const chatBox = document.getElementById("chat-box");
const userInput = document.getElementById("user-input");
const sendBtn = document.querySelector("button");

const API_URL = "http://localhost:3000/chat";
let chatHistory = JSON.parse(localStorage.getItem("chatHistory")) || [];

if (chatHistory.length > 50) chatHistory = chatHistory.slice(-50);

chatHistory.forEach((msg) => addMessage(msg.role, msg.content));

function addMessage(role, text) {
  const message = document.createElement("div");
  message.classList.add(
    "message",
    role === "user" ? "user-message" : "bot-message"
  );
  message.textContent = text;
  chatBox.appendChild(message);
  chatBox.scrollTop = chatBox.scrollHeight;

  chatHistory.push({ role, content: text });
  if (chatHistory.length > 50) chatHistory = chatHistory.slice(-50);
  localStorage.setItem("chatHistory", JSON.stringify(chatHistory));
}

function sendMessage() {
  const userMessage = userInput.value.trim();
  if (!userMessage) return;

  addMessage("user", userMessage);
  userInput.value = "";

  fetch(API_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ messages: chatHistory.slice(-10) }),
  })
    .then((response) => response.json())
    .then((data) => {
      console.log("Response from Server:", data);
      const botReply =
        data?.choices?.[0]?.message?.content || "I'm not sure how to respond.";
      addMessage("bot", botReply);
    })
    .catch((error) => {
      console.error("Fetch error:", error.message || error);
      addMessage("bot", "Sorry, something went wrong.");
    });
}

sendBtn.addEventListener("click", sendMessage);
userInput.addEventListener("keypress", (e) => {
  if (e.key === "Enter") sendMessage();
});
