from fastapi import FastAPI
import uvicorn


app = FastAPI()


@app.get("/")
async def test_yo() -> dict:
    return {"message": "hello world"}


if __name__ == "__main__":
    uvicorn.run("main:app", port=8001)
