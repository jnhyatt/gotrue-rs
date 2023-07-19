# (DONE) Endpoints

GoTrue exposes the following endpoints:


### **GET `/settings`**

Returns the publicly available settings for this gotrue instance.

<details>
### Response

```json
{
  "external": {
    "apple": true,
    "azure": true,
    "bitbucket": true,
    "discord": true,
    "facebook": true,
    "figma": true,
    "github": true,
    "gitlab": true,
    "google": true,
    "keycloak": true,
    "linkedin": true,
    "notion": true,
    "slack": true,
    "spotify": true,
    "twitch": true,
    "twitter": true,
    "workos": true
  },
  "disable_signup": false,
  "autoconfirm": false
}
```
</details>

