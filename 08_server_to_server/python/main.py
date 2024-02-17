from fastapi import FastAPI
import uvicorn
import requests

app = FastAPI()


@app.get("/fastapiData")
def another_route() -> dict:
    return {"message": [1, 2, 3, 4, 5]}


@app.get("/request-express")
async def yooo():
    response = requests.get(url="http://localhost:8080/express-data")
    response_data = response.json()

    return response_data["isRunning"]


if __name__ == "__main__":
    uvicorn.run(app="main:app", port=8000, reload=True)
