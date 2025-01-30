const express = require("express");
const app = express();

app.use(express.urlencoded({ extended: true }));

app.get("/", (req, res) => {
  res.send(`
        <form id="loginForm" method="POST" action="/login">
            <input type="text" id="username" name="username" placeholder="Username" required />
            <input type="password" id="password" name="password" placeholder="Password" required />
            <button id="loginButton" type="submit">Login</button>
        </form>
           `);
});

app.post("/login", (req, res) => {
  const { username, password } = req.body;

  if (username === "jay" && password === "1234") {
    res.redirect("/success");
  } else {
    res.redirect("/failure");
  }
});

app.get("/success", (req, res) => {
  res.send("<h1>Successfully logged in</h1>");
});

app.get("/failure", (req, res) => {
  res.send("<h1>Incorrect credentials!!</h1>");
});

const PORT = 3000;
app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
