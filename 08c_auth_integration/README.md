# Guide on how to set it up

I started by generating some frontent code using the their template:
<img src="screenshots/01.png" alt="alt text" width="400"/>

I chose to use vue:
<img src="screenshots/02.png" alt="alt text" width="400"/>

After this i could download the samlple app:
<img src="screenshots/03.png" alt="alt text" width="400"/>
<img src="screenshots/04.png" alt="alt text" width="400"/>

using this info i had to update the `auth_config.json` file:

```json
{
    "domain": "dev-knalxdoz8qzbe5a7.eu.auth0.com",
  "clientId": "XRj9VlkddDgd02e0RXMCHOcZABTyGVn0"
}
```


i also needed to add the valid callback urls:
<img src="screenshots/05.png" alt="alt text" width="400"/>

now i can view users created from the website:
<img src="screenshots/06.png" alt="alt text" width="400"/>

