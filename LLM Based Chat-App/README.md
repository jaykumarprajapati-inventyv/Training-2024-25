# 🤖 Chatbot App

A **simple yet powerful AI-powered chatbot** using **Node.js**, **Express**, and **OpenRouter API** for natural language processing. The frontend is built with **HTML, CSS, and JavaScript**, providing an interactive and user-friendly experience. The chat history is stored in **localStorage**, ensuring messages are retained between sessions.

---

## ✨ Features
- **AI-powered chat** using OpenRouter API (GPT models).
- **Chat history persistence** (stored in `localStorage`).
- **User-friendly UI** with real-time chat updates.
- **RESTful API backend** powered by Express.
- **CORS-enabled** server to allow frontend communication.

---

## 🚀 Technologies Used
### 🖥️ Frontend:
- **HTML** – Structure
- **CSS** – Styling
- **JavaScript** – Logic & API Communication

### 🛠️ Backend:
- **Node.js** – Server-side runtime
- **Express.js** – Web framework
- **Axios** – HTTP requests
- **CORS** – Handling cross-origin requests
- **dotenv** – Managing API keys securely

---

## 📂 Folder Structure
```
LLM Based Chat-App/
├── frontend/
│   ├── index.html       # Chat UI
│   ├── style.css        # Styling
│   ├── script.js        # Frontend Logic
│
├── backend/
│   ├── server.js        # Express Server
│   ├── .env             # API Key Storage
│   
├── package.json     # Dependencies
├── package-lock.json   # Lock Dependencies
└── README.md            # Documentation
```

---

## 🔧 Installation & Setup
### 📌 Prerequisites
- **Node.js** installed
- **Live Server** extension for frontend testing

### ⚙️ Backend Setup
1. **Clone the repository**
   ```sh
   git clone url
   cd LLM Based Chat-App/backend
   ```

2. **Install dependencies**
   This project uses a `.gitignore` file to exclude unnecessary files from Git tracking.  
   Make sure to run the following command after cloning the repository:
    ```sh
     npm install
    ```
    
3. **Create `.env` file** and add your OpenRouter API key:
   ```sh
   OPENROUTER_API_KEY=your-api-key-here
   ```

4. **Run the server**
   ```sh
   node server.js
   ```
   **Server will start on** `http://localhost:3000`

### 🎨 Frontend Setup
1. Open `frontend/index.html` using **Live Server** in VS Code.
2. The chatbot UI should load in the browser.

---

## 📌 Usage
1. Type a message in the input field.
2. Click **Send** or press **Enter**.
3. The AI chatbot responds based on OpenRouter API.
4. **Chat history is saved** and persists between refreshes.

---


