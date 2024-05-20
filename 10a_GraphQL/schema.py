import asyncio
from typing import List, AsyncGenerator
import strawberry
from strawberry import Schema

from models import College, Student
from db import get_db


@strawberry.type
class CollegeType:
    id: int
    name: str
    location: str


@strawberry.type
class StudentType:
    id: int
    name: str
    age: int
    college_id: int


@strawberry.type
class Query:
    @strawberry.field
    async def colleges(self) -> List[CollegeType]:
        db = get_db()
        colleges = db.query(College).all()
        return [
            CollegeType(id=college.id, name=college.name, location=college.location)
            for college in colleges
        ]

    @strawberry.field
    async def students(self, name: str | None = None) -> List[StudentType]:
        db = get_db()
        if name:
            students = db.query(Student).filter(Student.name == name).all()
        else:
            students = db.query(Student).all()
        return [
            StudentType(
                id=student.id,
                name=student.name,
                age=student.age,
                college_id=student.college_id,
            )
            for student in students
        ]


@strawberry.type
class Mutation:
    @strawberry.mutation
    async def create_college(self, name: str, location: str) -> CollegeType:
        db = get_db()
        college = College(name=name, location=location)
        db.add(college)
        db.commit()
        db.refresh(college)
        return CollegeType(id=college.id, name=college.name, location=college.location)

    @strawberry.mutation
    async def create_student(self, name: str, age: int, college_id: int) -> StudentType:
        db = get_db()
        college = db.query(College).filter(College.id == college_id).first()
        if not college:
            raise ValueError("College not found")
        student = Student(name=name, age=age, college_id=college_id)
        db.add(student)
        db.commit()
        db.refresh(student)
        return StudentType(
            id=student.id,
            name=student.name,
            age=student.age,
            college_id=student.college_id,
        )
