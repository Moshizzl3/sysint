import express from "express";
import http from "http";
const app = express();

app.get("/requestFastApi", async (req, res) => {
  const url = "http://localhost:8000/fastapiData";

  const response = await fetch(url);
  const response_json = await response.json();
  console.log(response_json);
  res.status(200).send({ message: response_json.message });
});

app.get("/express-data", (req, res) => {
  res.status(200).send({ isRunning: "true" });
});

const PORT = 8080;
app.listen(8080, console.log("Server is running on port", PORT));

