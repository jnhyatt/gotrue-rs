# Endpoints

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


### **POST `/admin/users/<user_id>`**

Creates (POST) the user based on the `user_id` specified. The `ban_duration` field accepts the following time units: "ns", "us", "ms", "s", "m", "h". See [`time.ParseDuration`](https://pkg.go.dev/time#ParseDuration) for more details on the format used.

<details>
### Request

```js
headers:
{
  "Authorization": "Bearer eyJhbGciOiJI...M3A90LCkxxtX9oNP9KZO" // requires a role claim that can be set in the GOTRUE_JWT_ADMIN_ROLES env var
}

body:
{
  "role": "test-user",
  "email": "email@example.com",
  "phone": "12345678",
  "password": "secret", // only if type = signup
  "email_confirm": true,
  "phone_confirm": true,
  "user_metadata": {},
  "app_metadata": {},
  "ban_duration": "24h" or "none" // to unban a user
}
```
</details>

## **PUT `/admin/users/<user_id>`**

Updates (PUT) the user based on the `user_id` specified. The `ban_duration` field accepts the following time units: "ns", "us", "ms", "s", "m", "h". See [`time.ParseDuration`](https://pkg.go.dev/time#ParseDuration) for more details on the format used.


<details>
### Request

```js
headers:
{
  "Authorization": "Bearer eyJhbGciOiJI...M3A90LCkxxtX9oNP9KZO" // requires a role claim that can be set in the GOTRUE_JWT_ADMIN_ROLES env var
}

body:
{
  "role": "test-user",
  "email": "email@example.com",
  "phone": "12345678",
  "password": "secret", // only if type = signup
  "email_confirm": true,
  "phone_confirm": true,
  "user_metadata": {},
  "app_metadata": {},
  "ban_duration": "24h" or "none" // to unban a user
}
```
</details>

## **POST `/admin/generate_link`**

Returns the corresponding email action link based on the type specified. Among other things, the response also contains the query params of the action link as separate JSON fields for convenience (along with the email OTP from which the corresponding token is generated).

<details>
### Request

```js
headers:
{
  "Authorization": "Bearer eyJhbGciOiJI...M3A90LCkxxtX9oNP9KZO" // admin role required
}

body:
{
  "type": "signup" or "magiclink" or "recovery" or "invite",
  "email": "email@example.com",
  "password": "secret", // only if type = signup
  "data": {
    ...
  }, // only if type = signup
  "redirect_to": "https://supabase.io" // Redirect URL to send the user to after an email action. Defaults to SITE_URL.

}
```

Returns

```js
{
  "action_link": "http://localhost:9999/verify?token=TOKEN&type=TYPE&redirect_to=REDIRECT_URL",
  "email_otp": "EMAIL_OTP",
  "hashed_token": "TOKEN",
  "verification_type": "TYPE",
  "redirect_to": "REDIRECT_URL",
  ...
}
```
</details>

## **POST `/signup`**

Register a new user with an email and password.

<details>
```js
{
  "email": "email@example.com",
  "password": "secret"
}
```

returns:

```json
{
  "id": "11111111-2222-3333-4444-5555555555555",
  "email": "email@example.com",
  "confirmation_sent_at": "2016-05-15T20:49:40.882805774-07:00",
  "created_at": "2016-05-15T19:53:12.368652374-07:00",
  "updated_at": "2016-05-15T19:53:12.368652374-07:00"
}

// if sign up is a duplicate then faux data will be returned
// as to not leak information about whether a given email
// has an account with your service or not
```

Register a new user with a phone number and password.

```js
{
  "phone": "12345678", // follows the E.164 format
  "password": "secret"
}
```

Returns:

```json
{
  "id": "11111111-2222-3333-4444-5555555555555", // if duplicate sign up, this ID will be faux
  "phone": "12345678",
  "confirmation_sent_at": "2016-05-15T20:49:40.882805774-07:00",
  "created_at": "2016-05-15T19:53:12.368652374-07:00",
  "updated_at": "2016-05-15T19:53:12.368652374-07:00"
}
```

if AUTOCONFIRM is enabled and the sign up is a duplicate, then the endpoint will return:

```json
{
  "code":400,
  "msg":"User already registered"
}
```
</details>

## **POST `/invite`**

Invites a new user with an email.
This endpoint requires the `service_role` or `supabase_admin` JWT set as an Auth Bearer header:

<details>
e.g.

```json
headers: {
  "Authorization" : "Bearer eyJhbGciOiJI...M3A90LCkxxtX9oNP9KZO"
}
```

```json
{
  "email": "email@example.com"
}
```

Returns:

```json
{
  "id": "11111111-2222-3333-4444-5555555555555",
  "email": "email@example.com",
  "confirmation_sent_at": "2016-05-15T20:49:40.882805774-07:00",
  "created_at": "2016-05-15T19:53:12.368652374-07:00",
  "updated_at": "2016-05-15T19:53:12.368652374-07:00",
  "invited_at": "2016-05-15T19:53:12.368652374-07:00"
}
```
</details>

## **POST `/verify`**

Verify a registration or a password recovery. Type can be `signup` or `recovery` or `invite`
and the `token` is a token returned from either `/signup` or `/recover`.

<details>

```json
{
  "type": "signup",
  "token": "confirmation-code-delivered-in-email"
}
```

`password` is required for signup verification if no existing password exists.

Returns:

```json
{
  "access_token": "jwt-token-representing-the-user",
  "token_type": "bearer",
  "expires_in": 3600,
  "refresh_token": "a-refresh-token",
  "type": "signup | recovery | invite"
}
```

Verify a phone signup or sms otp. Type should be set to `sms`.

```json
{
  "type": "sms",
  "token": "confirmation-otp-delivered-in-sms",
  "redirect_to": "https://supabase.io",
  "phone": "phone-number-sms-otp-was-delivered-to"
}
```

Returns:

```json
{
  "access_token": "jwt-token-representing-the-user",
  "token_type": "bearer",
  "expires_in": 3600,
  "refresh_token": "a-refresh-token"
}
```
</details>

## **GET `/verify`**

Verify a registration or a password recovery. Type can be `signup` or `recovery` or `magiclink` or `invite`
and the `token` is a token returned from either `/signup` or `/recover` or `/magiclink`.

<details>

query params:

```json
{
  "type": "signup",
  "token": "confirmation-code-delivered-in-email",
  "redirect_to": "https://supabase.io"
}
```

User will be logged in and redirected to:

```plaintext
SITE_URL/#access_token=jwt-token-representing-the-user&token_type=bearer&expires_in=3600&refresh_token=a-refresh-token&type=invite
```

Your app should detect the query params in the fragment and use them to set the session (supabase-js does this automatically)

You can use the `type` param to redirect the user to a password set form in the case of `invite` or `recovery`,
or show an account confirmed/welcome message in the case of `signup`, or direct them to some additional onboarding flow
</details>

## **POST `/otp`**

One-Time-Password. Will deliver a magiclink or sms otp to the user depending on whether the request body contains an "email" or "phone" key.

If `"create_user": true`, user will not be automatically signed up if the user doesn't exist.

<details>

```json
{
  "phone": "12345678" // follows the E.164 format
  "create_user": true
}

OR

// exactly the same as /magiclink
{
  "email": "email@example.com"
  "create_user": true
}
```

Returns:

```json
{}
```
</details>

## **POST `/magiclink`** (recommended to use /otp instead. See above.)

Magic Link. Will deliver a link (e.g. `/verify?type=magiclink&token=fgtyuf68ddqdaDd`) to the user based on
email address which they can use to redeem an access_token.

By default Magic Links can only be sent once every 60 seconds

<details>

```json
{
  "email": "email@example.com"
}
```

Returns:

```json
{}
```

when clicked the magic link will redirect the user to `<SITE_URL>#access_token=x&refresh_token=y&expires_in=z&token_type=bearer&type=magiclink` (see `/verify` above)

</details>

## **POST `/recover`**

Password recovery. Will deliver a password recovery mail to the user based on
email address.

By default recovery links can only be sent once every 60 seconds

<details>
```json
{
  "email": "email@example.com"
}
```

Returns:

```json
{}
```

</details>

## **POST `/token`**

This is an OAuth2 endpoint that currently implements
the password and refresh_token grant types

<details>

query params:

```plaintext
?grant_type=password
```

body:

```json
// Email login
{
  "email": "name@domain.com",
  "password": "somepassword"
}

// Phone login
{
  "phone": "12345678",
  "password": "somepassword"
}
```

or

query params:

```plaintext
grant_type=refresh_token
```

body:

```json
{
  "refresh_token": "a-refresh-token"
}
```

Once you have an access token, you can access the methods requiring authentication
by settings the `Authorization: Bearer YOUR_ACCESS_TOKEN_HERE` header.

Returns:

```json
{
  "access_token": "jwt-token-representing-the-user",
  "token_type": "bearer",
  "expires_in": 3600,
  "refresh_token": "a-refresh-token"
}
```
</details>

## **GET `/user`**

Get the JSON object for the logged in user (requires authentication)

<details>

Returns:

```json
{
  "id": "11111111-2222-3333-4444-5555555555555",
  "email": "email@example.com",
  "confirmation_sent_at": "2016-05-15T20:49:40.882805774-07:00",
  "created_at": "2016-05-15T19:53:12.368652374-07:00",
  "updated_at": "2016-05-15T19:53:12.368652374-07:00"
}
```
</details>

## **PUT `/user`**

Update a user (Requires authentication). Apart from changing email/password, this
method can be used to set custom user data. Changing the email will result in a magiclink being sent out.

<details>

```json
{
  "email": "new-email@example.com",
  "password": "new-password",
  "phone": "+123456789",
  "data": {
    "key": "value",
    "number": 10,
    "admin": false
  }
}
```

Returns:

```json
{
  "id": "11111111-2222-3333-4444-5555555555555",
  "email": "email@example.com",
  "email_change_sent_at": "2016-05-15T20:49:40.882805774-07:00",
  "phone": "+123456789",
  "phone_change_sent_at": "2016-05-15T20:49:40.882805774-07:00",
  "created_at": "2016-05-15T19:53:12.368652374-07:00",
  "updated_at": "2016-05-15T19:53:12.368652374-07:00"
}
```

If `GOTRUE_SECURITY_UPDATE_PASSWORD_REQUIRE_REAUTHENTICATION` is enabled, the user will need to reauthenticate first.

```json
{
  "password": "new-password",
  "nonce": "123456"
}
```
</details>

## **GET `/reauthenticate`**

Sends a nonce to the user's email (preferred) or phone. This endpoint requires the user to be logged in / authenticated first. The user needs to have either an email or phone number for the nonce to be sent successfully.

<details>

```json
headers: {
  "Authorization" : "Bearer eyJhbGciOiJI...M3A90LCkxxtX9oNP9KZO"
}
```
</details>

## **POST `/logout`**

Logout a user (Requires authentication).

This will revoke all refresh tokens for the user. Remember that the JWT tokens
will still be valid for stateless auth until they expires.

## **GET `/authorize`**

Get access_token from external oauth provider

<details>

query params:

```
provider=apple | azure | bitbucket | discord | facebook | figma | github | gitlab | google | keycloak | linkedin | notion | slack | spotify | twitch | twitter | workos

scopes=<optional additional scopes depending on the provider (email and name are requested by default)>
```

Redirects to provider and then to `/callback`

For apple specific setup see: <https://github.com/supabase/gotrue#apple-oauth>

</details>

## **GET `/callback`**

External provider should redirect to here

<details>

Redirects to 
```plaintext
<GOTRUE_SITE_URL>#access_token=<access_token>&refresh_token=<refresh_token>&provider_token=<provider_oauth_token>&expires_in=3600&provider=<provider_name>
```
If additional scopes were requested then `provider_token` will be populated, you can use this to fetch additional data from the provider or interact with their services

</details>