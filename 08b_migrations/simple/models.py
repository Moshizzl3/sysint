from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import Column, DateTime, String, Integer, func
from sqlalchemy import MetaData

metadata = MetaData()
Base = declarative_base(metadata=metadata)


class Student(Base):
    __tablename__ = "students"

    id = Column(Integer, primary_key=True)
    name = Column(String(60))
    age = Column(Integer)
    note = Column(String(200))
    create_at = Column(DateTime, default=func.now())

    def __repr__(self):
        return f"id: {self.id}, name: {self.name}"


class College(Base):
    __tablename__ = "college"

    id = Column(Integer, primary_key=True)
    name = Column(String(60))
    location = Column(String(60))
    create_at = Column(DateTime, default=func.now())

    def __repr__(self):
        return f"id: {self.id}, name: {self.name}"
