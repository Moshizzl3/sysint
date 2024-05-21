from fastapi import FastAPI, HTTPException
import stripe
import uvicorn
from dotenv import load_dotenv
import os

load_dotenv()
app = FastAPI()
PORT = 8000
stripe.api_key = os.getenv("STRIPE_API_KEY")


@app.post("/intent/")
async def create_payment(amount: int, currency: str, token: str, description: str):
    try:

        intent = stripe.PaymentIntent.create(
            amount=amount,
            currency=currency,
            description=description,
            payment_method_types=["card"],
            payment_method="pm_card_visa",
        )

        # Return a success response
        return {"status": "success", "intent": intent.id, "status": intent.status}

    except stripe.error.CardError as e:
        return {"status": "error", "message": str(e)}
    except stripe.error.StripeError as e:
        return {
            "status": "error",
            "message": "Something went wrong. Please try again later.",
            "error": e,
        }


@app.post("/confirmation/")
async def confirm_payment(intent: str):

    confirmation = stripe.PaymentIntent.confirm(
        intent=intent,
        payment_method="pm_card_visa",
        return_url="https://www.example.com",
    )

    return {"data": confirmation}


if __name__ == "__main__":
    uvicorn.run("main:app", port=PORT, reload=True)
