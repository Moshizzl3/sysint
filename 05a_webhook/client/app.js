import express from "express";

const app = express();

let lastOrder = "No order yet"; // Store the latest status here

app.use(express.static("public"));

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post("/status", (req, res) => {
  lastOrder = req.body; // Update the latest status
  console.log(req.body);
  res.sendStatus(200);
});

app.post("/ping", (req, res) => {
  console.log(req.body);
  res.sendStatus(200);
});

app.get("/latest-status", (req, res) => {
  res.json(lastOrder);
});

const PORT = 8080;
app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
