from fastapi import FastAPI, Request
import json

app = FastAPI()


@app.post("/githubwebhookjson")
async def github_webhook(request: Request):
    data = await request.body()
    print(json.loads(data))
    return


@app.post("/githubwebhookform")
async def github_webhook_form(request: Request):
    if request.headers.get("content-type") == "application/x-www-form-urlencoded":
        data = await request.form()
        print(data)
    return
