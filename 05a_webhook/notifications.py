import requests
from fastapi import HTTPException


def notify_subscriber(url, payload):

    try:

        response = requests.post(url=url, json=payload.dict())
        if response.status_code == 200:
            print(f"Notification sent to {url}")
        else:
            print(
                f"Failed to send notification to {url}, status code: {response.status_code}"
            )
    except Exception as e:
        print(e)
