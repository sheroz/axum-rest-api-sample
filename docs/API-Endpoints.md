# REST API Documentation

## Health Check (Public)

**Endpoint:** `GET /v1/health`

**Description:** Checks the health of the service.

---

## Version Check (Public)

**Endpoint:** `GET /v1/version`

**Description:** Retrieves the version of the service.

---

## Login

**Endpoint:** `POST /v1/auth/login`

**Description:** Authenticates a user and returns tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`

**Request Body:**

```json
{
    "username": "admin",
    "password_hash": "7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51"
}
```

---

## Root (Protected)

**Endpoint:** `GET /`

**Description:** Accesses the root endpoint.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

---

## Refresh Tokens

**Endpoint:** `POST /v1/auth/refresh`

**Description:** Refreshes the tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <refresh_token>`

---

## Logout

**Endpoint:** `POST /v1/auth/logout`

**Description:** Logs out the user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <refresh_token>`

---

## Revoke Tokens Issued to the User

**Endpoint:** `POST /v1/auth/revoke-user`

**Description:** Revokes tokens issued to the user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{ "user_id" : "617646a0-7437-48a0-bb03-a7aa830f8f81" }
```

---

## Revoke All Issued Tokens

**Endpoint:** `POST /v1/auth/revoke-all`

**Description:** Revokes all issued tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

---

## Cleanup Revoked Tokens

**Endpoint:** `POST /v1/auth/cleanup`

**Description:** Cleans up revoked tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

---

## List Users

**Endpoint:** `GET /v1/users`

**Description:** Lists all users.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## Get User by ID

**Endpoint:** `GET /v1/users/{user_id}`

**Description:** Retrieves a user by ID.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## Add a New User

**Endpoint:** `POST /v1/users`

**Description:** Adds a new user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
    "id": "917646a0-7437-48a0-bb03-a7aa830f8f81",
    "username": "admin2",
    "email": "admin2@admin.com",
    "password_hash": "7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51",
    "password_salt": "pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF"
}
```

---

## Update User

**Endpoint:** `PUT /v1/users/{user_id}`

**Description:** Updates a user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
    "id": "917646a0-7437-48a0-bb03-a7aa830f8f81",
    "username": "admin21",
    "email": "admin21@admin.com",
    "password_hash": "7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51",
    "password_salt": "pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF"
}
```

---

## Delete User

**Endpoint:** `DELETE /v1/users/{user_id}`

**Description:** Deletes a user.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## List Accounts

**Endpoint:** `GET /v1/accounts`

**Description:** Lists all accounts.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## Get Account by ID

**Endpoint:** `GET /v1/accounts/{account_id}`

**Description:** Retrieves an account by ID.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## Add a New Account

**Endpoint:** `POST /v1/accounts`

**Description:** Adds a new account.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
    "id": "72022566-44e2-44a4-bf07-485c6a56d506",
    "user_id": "917646a0-7437-48a0-bb03-a7aa830f8f81",
    "balance_cents": 0
}
```

---

## Update Account

**Endpoint:** `PUT /v1/accounts/{account_id}`

**Description:** Updates an account.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
    "id": "72022566-44e2-44a4-bf07-485c6a56d506",
    "user_id": "917646a0-7437-48a0-bb03-a7aa830f8f81",
    "balance_cents": 100
}
```

---

## Transaction: Transfer Money

**Endpoint:** `POST /v1/transactions/transfer`

**Description:** Transfers money between accounts.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
    "source_account_id": "9a7a8f34-a949-42e3-b552-e1818069003c",
    "destination_account_id": "94e331e9-d300-42db-a075-df6d49f81361",
    "amount_cents": 25
}
```

---

## Get Transaction by ID

**Endpoint:** `GET /v1/transactions/{transaction_id}`

**Description:** Retrieves a transaction by ID.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

---

### END
