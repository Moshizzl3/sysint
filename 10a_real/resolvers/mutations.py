import asyncio
from typing import List, AsyncGenerator
import strawberry
from strawberry import Schema

from models import Book, Author
from db import get_db


async def create_author(
    name: str,
) -> Author:
    db = get_db()
    author = Author(name=name)
    db.add(author)
    db.commit()
    db.refresh(author)
    return Author(id=author.id, name=author.name)


async def create_book(title: str, release_year: int, author_id: int) -> Book:
    db = get_db()
    author = db.query(Author).filter(Author.id == author_id).first()
    if not author:
        raise ValueError("Author not found")
    book = Book(title=title, release_year=release_year, author_id=author_id)
    db.add(book)
    db.commit()
    db.refresh(book)
    return Book(
        id=book.id,
        title=book.title,
        release_year=book.release_year,
        author_id=book.author_id,
    )
