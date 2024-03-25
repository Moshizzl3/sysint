from sqlalchemy import create_engine, Column, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker, Session
import enum
from sqlalchemy import Enum


class SubscriptionStatus(enum.Enum):
    PENDING = "pending"
    PROCESSING = "processing"
    COMPLETED = "completed"
    CANCELLED = "cancelled"


SQLALCHEMY_DATABASE_URL = "sqlite:///./test.db"
engine = create_engine(
    SQLALCHEMY_DATABASE_URL, connect_args={"check_same_thread": False}
)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()


class Subscription(Base):
    __tablename__ = "subscriptions"

    order_number = Column(String, primary_key=True, index=True)
    url = Column(String, index=True)
    status = Column(Enum(SubscriptionStatus), default=SubscriptionStatus.PENDING)


Base.metadata.create_all(bind=engine)


# Dependency
def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


def update_order_status(db: Session, order_number: str, new_status: str):
    order = (
        db.query(Subscription).filter(Subscription.order_number == order_number).first()
    )
    if order:
        order.status = new_status
        db.commit()
        db.refresh(order)
        return order
    return None

