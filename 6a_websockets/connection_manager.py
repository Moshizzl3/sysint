from fastapi import WebSocket


class ConnectionManager:
    def __init__(self):
        self.active_connections: dict[str, WebSocket] = {}

    async def connect(self, websocket: WebSocket, id: str):
        await websocket.accept()
        self.active_connections.update({id: websocket})

    def disconnect(self, id: str):
        if self.active_connections.get(id):
            self.active_connections.pop(id)

    async def send_personal_message(self, message: str, id: str):
        websocket = self.active_connections.get(id)
        print(websocket)
        if websocket:
            await websocket.send_text(message)

    async def broadcast(self, message: str):
        for connection in self.active_connections.values():
            await connection.send_text(message)
