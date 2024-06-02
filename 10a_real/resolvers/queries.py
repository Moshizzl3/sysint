import asyncio
from turtle import title
from typing import List, AsyncGenerator
import strawberry
from strawberry import Schema

from models import Book, Author
from db import get_db


async def books() -> List[Book]:
    db = get_db()
    books = db.query(Book).all()
    return [book for book in books]


async def authors(name: str | None = None) -> List[Author]:
    db = get_db()
    if name:
        authors = db.query(Author).filter(Author.name == name).all()
    else:
        authors = db.query(Author).all()
    return [
        Author(
            id=author.id,
            name=author.name,
        )
        for author in authors
    ]
