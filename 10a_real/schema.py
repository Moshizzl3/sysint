import asyncio
from typing import List, AsyncGenerator
import strawberry
from strawberry import Schema

from resolvers.mutations import create_author, create_book
from resolvers.queries import books, authors

import strawberry


@strawberry.type
class Author:
    id: strawberry.ID
    name: str | None


@strawberry.type
class Book:
    id: strawberry.ID
    title: str | None
    release_year: int | None
    author_id: strawberry.ID


@strawberry.type
class ErrorMessage:
    message: str | None
    error_code: int | None


@strawberry.type
class SuccessMessage:
    message: str | None


@strawberry.type
class Query:
    books: list[Book | None] | None = strawberry.field(
        description="Get all books", resolver=books
    )
    authors: list[Author | None] | None = strawberry.field(
        description="Get all authors", resolver=authors
    )
    # author: Author | None = strawberry.field(
    #     description="Get an author by id", resolver=authors
    # )


@strawberry.type
class Mutation:
    create_book: Book | None = strawberry.field(
        description="Create a new book", resolver=create_book
    )
    create_author: Author | None = strawberry.field(
        description="Create a new author", resolver=create_author
    )

    delete_book: SuccessMessage | None = strawberry.field(
        description="Delete a book by id"
    )


@strawberry.type
class Subscription:
    book_added: Book | None = strawberry.field(
        description="Updates when a new book has been added"
    )


schema = strawberry.Schema(query=Query, mutation=Mutation, subscription=Subscription)
