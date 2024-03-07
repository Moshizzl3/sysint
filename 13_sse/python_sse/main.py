from fastapi import FastAPI, Request
from fastapi.responses import StreamingResponse
from fastapi.templating import Jinja2Templates
from datetime import datetime
import asyncio

app = FastAPI()


templates = Jinja2Templates(directory="templates")


@app.get("/")
async def serve_root_page(request: Request):
    return templates.TemplateResponse("index.html", {"request": request})


async def date_generator():
    while True:
        now = datetime.utcnow().strftime("%Y-%m-%d %H:%M:%S")
        yield f"data: {now}\n\n"
        await asyncio.sleep(1)


@app.get("/sse")
def sse():
    return StreamingResponse(date_generator(), media_type="text/event-stream")
