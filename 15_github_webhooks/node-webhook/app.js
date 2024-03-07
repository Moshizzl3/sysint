import express from "express";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post("/githubwebhookjson", (req, res) => {
  console.log(req.body);
  res.sendStatus(204);
});

app.post("/githubwebhookform", (req, res) => {
  console.log(req.body);
  res.sendStatus(204);
});

app.get("/test", (req, res) => {
  res.send("hello");
});

app.listen(8080, () => {
  console.log("server running on: ", 8080);
});
