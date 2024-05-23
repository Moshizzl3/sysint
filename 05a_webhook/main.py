from fastapi import FastAPI, Depends, HTTPException
import uvicorn
from pydantic import BaseModel, Field
from typing import TypeVar, Generic
from db import (
    get_db,
    Subscription,
    SubscriptionStatus,
    update_order_status,
    get_all_urls,
)
from notifications import notify_subscriber
from sqlalchemy.orm import Session
from datetime import datetime

app = FastAPI()


class SubscriptionRequestDTO(BaseModel):
    url: str
    order_number: str


class PingRequestDTO(BaseModel):
    url: str | None


class PingResponseDTO(BaseModel):
    message: str = Field(default="Ping from backend")
    time: str = str(datetime.now())


class StatusUpdateDTO(BaseModel):
    status: SubscriptionStatus = Field(..., description="Status of the subscription")
    order_number: str


T = TypeVar("T")


class MessageOk(BaseModel):
    status: str = "Success"
    message: str = "Object was created."


class MessageFail(BaseModel):
    status: str = "Fail"
    message: str = "Object was not created."


class ResponseDataDTO(BaseModel, Generic[T]):
    data: T


@app.post("/subscription", response_model=ResponseDataDTO)
async def subscribe_to_webhook(
    input_dto: SubscriptionRequestDTO, db: Session = Depends(get_db)
) -> ResponseDataDTO[MessageOk]:
    """Subscribe to webhook."""
    new_subscription = Subscription(
        url=input_dto.url,
        order_number=input_dto.order_number,
        status=SubscriptionStatus.PROCESSING,
    )
    db.add(new_subscription)
    db.commit()
    return ResponseDataDTO(data=MessageOk())


@app.post("/update_order_status")
async def update_order_status_endpoint(
    input_dto: StatusUpdateDTO, db: Session = Depends(get_db)
):
    """Endpoint for updating order. This triggers webhook event.
    Valid inputs:
        - PENDING
        - PROCESSING
        - COMPLETED
        - CANCELLED
    """

    input_dto.status = str(input_dto.status.name).upper()
    order = update_order_status(db, input_dto.order_number, input_dto.status)

    if order:
        notify_subscriber(
            order.url,
            input_dto,
        )
        return MessageOk(message="Order updated and subscribers notified")
    else:
        raise HTTPException(status_code=404, detail="Order not found")


@app.post("/ping")
async def ping_url(
    input_dto: PingRequestDTO, db: Session = Depends(get_db)
) -> MessageOk:
    """Endpoint for pinging the url. If url is None, a ping will go out to all urls."""

    if input_dto.url is None:
        urls = get_all_urls(db=db)

        for url in urls:
            notify_subscriber(url, PingResponseDTO())

    notify_subscriber(input_dto.url, PingResponseDTO())
    return MessageOk(message="Ping 🔔")


if __name__ == "__main__":
    uvicorn.run(
        app="main:app",
        host="127.0.0.1",
        port=8000,
        reload=True,
    )
