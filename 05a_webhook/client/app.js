import express from "express";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post("/status", (req, res) => {
  console.log(req.body);
  res.sendStatus(200);
});


app.listen(8080, () => {
  console.log("server running on: ", 8080);
});
