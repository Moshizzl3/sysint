import strawberry
import uvicorn

from fastapi import FastAPI
from strawberry.fastapi import GraphQLRouter
import schema as my_schema

app = FastAPI()
PORT = 8000


@app.get("/")
async def index():
    return {"message": "Welcome to the books API"}


schema = strawberry.Schema(my_schema.Query, my_schema.Mutation)

graphql_app = GraphQLRouter(schema)  # type: ignore

app.include_router(graphql_app, prefix="/graphql")

if __name__ == "__main__":
    uvicorn.run("main:app", port=PORT, reload=True)
