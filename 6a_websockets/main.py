from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.responses import HTMLResponse
from connection_manager import ConnectionManager
import uvicorn
import json

app = FastAPI()
PORT = 8000

manager = ConnectionManager()


html = """
<!DOCTYPE html>
<html>
    <head>
        <title>Chat</title>
    </head>
    <body>
        <h1>WebSocket Chat</h1>
        <form id="connectForm" onsubmit="connectWebSocket(event)">
            <input type="text" id="userId" placeholder="Enter your ID" autocomplete="off"/>
            <button type="submit">Connect</button>
        </form>
        <form id="messageForm" onsubmit="sendMessage(event)" style="display:none;">
            <input type="text" id="messageText" placeholder="Enter your message" autocomplete="off"/>
            <input type="text" id="targetUserId" placeholder="Enter target user ID" autocomplete="off"/>
            <button>Send</button>
        </form>
        <ul id='messages'></ul>
        <script>
            let ws;
            function connectWebSocket(event) {
                event.preventDefault();
                const userId = document.getElementById("userId").value;
                ws = new WebSocket(`ws://localhost:8000/ws/message?user_id=${userId}`);
                ws.onmessage = function(event) {
                    const messages = document.getElementById('messages');
                    const message = document.createElement('li');
                    const content = document.createTextNode(event.data);
                    message.appendChild(content);
                    messages.appendChild(message);
                };
                document.getElementById("connectForm").style.display = 'none';
                document.getElementById("messageForm").style.display = 'block';
            }
            function sendMessage(event) {
                event.preventDefault();
                const input = document.getElementById("messageText");
                const targetUserId = document.getElementById("targetUserId").value;
                const message = `${targetUserId}:${input.value}`;
                ws.send(JSON.stringify({
                    user_id: targetUserId,
                    message: message
                    }));
                input.value = '';
            }
        </script>
    </body>
</html>
"""


@app.get("/")
async def get():
    return HTMLResponse(html)


@app.websocket("/ws/message")
async def websocket_endpoint_personal_message(websocket: WebSocket, user_id: str):
    await manager.connect(websocket=websocket, id=user_id)
    try:
        while True:
            data = await websocket.receive_text()
            json_data = json.loads(data)
            if not json_data["user_id"]:
                print("broadcast")
                await manager.broadcast(f"{user_id}: {json_data['message']}")
            else:
                print("personal")
                await manager.send_personal_message(
                    f"{user_id}: {json_data['message']}", json_data["user_id"]
                )
    except WebSocketDisconnect:
        manager.disconnect(user_id)
        await manager.broadcast(f"Client disconnected")


if __name__ == "__main__":
    uvicorn.run("main:app", port=PORT, reload=True)
