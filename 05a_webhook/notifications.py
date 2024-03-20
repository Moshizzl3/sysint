import httpx


async def notify_subscriber(url, payload):
    try:
        async with httpx.AsyncClient() as client:
            response = await client.post(url, json=payload)
            if response.status_code == 200:
                print(f"Notification sent to {url}")
            else:
                print(
                    f"Failed to send notification to {url}, status code: {response.status_code}"
                )
    except httpx.RequestError as e:
        print(f"Error sending notification to {url}: {str(e)}")
