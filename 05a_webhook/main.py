from fastapi import FastAPI, Depends, HTTPException
import uvicorn
from pydantic import BaseModel
from typing import TypeVar, Generic
from db import get_db, Subscription, SubscriptionStatus, update_order_status
from notifications import notify_subscriber
from sqlalchemy.orm import Session

app = FastAPI()


class SubscriptionRequestDTO(BaseModel):
    url: str
    order_number: str


class StatusUpdateDTO(BaseModel):
    status: str
    order_number: str


T = TypeVar("T")


class MessageOk(BaseModel):
    status: str = "Success"
    message: str = "Object was created."


class MessageFail(BaseModel):
    status: str = "Fail"
    message: str = "Object was not created."


class ResponseDataDTO(Generic[T], BaseModel):
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
    print("hello")
    return ResponseDataDTO(data=MessageOk())


@app.post("/update_order_status")
async def update_order_status_endpoint(
    input_dto: StatusUpdateDTO, db: Session = Depends(get_db)
):
    order = update_order_status(db, input_dto.order_number, input_dto.status)

    if order:
        notify_subscriber(
            order.url,
            input_dto,
        )
        return {"message": "Order updated and subscribers notified"}
    else:
        raise HTTPException(status_code=404, detail="Order not found")


if __name__ == "__main__":
    uvicorn.run(
        app="main:app",
        host="127.0.0.1",
        port=8000,
        reload=True,
    )
