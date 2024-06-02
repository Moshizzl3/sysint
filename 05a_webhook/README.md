# FastAPI Webhook System

This app allows you to subscribe to webhooks and receive updates on order status changes.
You can also test the connectivity by pinging the endpoint.


## Usage

Go to: <https://webhookapp420.azurewebsites.net/docs>

### Subscribing to a Webhook

To subscribe to a webhook, send a POST request to `/subscription` with the following JSON payload:

```json
{
  "url": "<your_callback_url>",
  "order_number": "<your_order_number>"
}
```

This will register your URL to receive updates about the specified order.

### Updating Order Status

To trigger an update, send a Patch request to /order-status with the following JSON payload:

```json
{
  "status": "<new_status>",
  "order_number": "<your_order_number>"
}
```

Valid status values are:

- PENDING
- PROCESSING
- COMPLETED
- CANCELLED

This will update the order status and notify the subscribed URLs about the change.

### Pinging the urls

You can test your webhook by sending a POST request to /ping with the following JSON payload.

### Delete url from webhook
You can delete a specific url, by using the delete endpoint, and passing a url.


This will send a ping to the specified URL.

If url is null, a ping will be sent to all urls subscribed.
