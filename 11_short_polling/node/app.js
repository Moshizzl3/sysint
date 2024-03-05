import express from "express";

const app = express();

app.use(express.static("public"));

const randomNumbers = [];

app.get("/random-numbers", (req, res) => {
  res.send({ data: randomNumbers });
});

app.get("/simulate-numbers", (req, res) => {
  const newNumber = getRandomInt(3, 1001);
  randomNumbers.push(newNumber);
  res.send({ data: randomNumbers });
});

function getRandomInt(min, max) {
  return Math.floor(Math.random() * (max - min) + min);
}

const PORT = 8080;
app.listen(PORT, () => console.log("running on: ", PORT));
