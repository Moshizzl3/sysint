import express from "express";

const app = express();

let lastOrder = "No order yet"; // Store the latest status here

app.use(express.static("public"));

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post("/status", (req, res) => {
  lastOrder = req.body; // Update the latest status
  res.sendStatus(200);
});

app.get("/latest-status", (req, res) => {
  res.json(lastOrder);
});

app.listen(80, () => {
  console.log("Server running on port 80");
});
