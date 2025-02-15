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

## Errors

### The possible error codes and description

- `authentication_wrong_credentials`: The provided credentials are incorrect.
- `authentication_missing_credentials`: Required authentication credentials are missing.
- `authentication_token_creation_error`: There was an error creating the authentication token.
- `authentication_invalid_token`: The provided authentication token is invalid.
- `authentication_revoked_tokens_inactive`: The provided token has been revoked and is inactive.
- `authentication_forbidden`: The user does not have permission to access the requested resource.
- `user_not_found`: The specified user was not found.
- `transaction_not_found`: The specified transaction was not found.
- `transfer_insufficient_funds`: The source account does not have sufficient funds for the transfer.
- `transfer_source_account_not_found`: The source account for the transfer was not found.
- `transfer_destination_account_not_found`: The destination account for the transfer was not found.
- `transfer_accounts_are_same`: The source and destination accounts for the transfer are the same.
- `resource_not_found`: The requested resource was not found.
- `api_version_error`: There is an error with the API version.
- `database_error`: There was an error with the database operation.
- `redis_error`: There was an error with the Redis operation.

### The possible error kinds and description

- `authentication_error`: An error occurred during the authentication process.
- `resource_not_found`: The requested resource could not be found.
- `validation_error`: There was a validation error with the provided data.
- `database_error`: An error occurred with the database operation.
- `redis_error`: An error occurred with the Redis operation.

### API error response samples

```json
 {
   "status": 404,
   "errors": [
     {
         "code": "user_not_found",
         "kind": "resource_not_found",
         "message": "user not found: 12345",
         "description": "user with the ID '12345' does not exist in our records",
         "detail": { "user_id": "12345" },
         "reason": "must be an existing user",
         "instance": "/api/v1/users/12345",
         "trace_id": "3d2b4f2d00694354a00522fe3bb86158",
         "timestamp": "2024-01-19T16:58:34.123+0000",
         "help": "please check if the user ID is correct or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information",
         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
     }
   ]
 }
```

```json
 {
   "status": 422,
   "errors": [
     {
         "code": "transfer_insufficient_funds",
         "kind": "validation_error",
         "message": "source account does not have sufficient funds for the transfer",
         "reason": "source account balance must be sufficient to cover the transfer amount",
         "instance": "/api/v1/transactions/transfer",
         "trace_id": "fbb9fdf5394d4abe8e42b49c3246310b",
         "timestamp": "2024-01-19T16:58:35.225+0000",
         "help": "please check the source account balance or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information",
         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
     },
     {
         "code": "transfer_destination_account_not_found",
         "kind": "validation_error",
         "message": "destination account not found: d424cfe9-c042-41db-9a8e-8da5715fea10",
         "detail": { "destination_account_id": "d424cfe9-c042-41db-9a8e-8da5715fea10" },
         "reason": "must be an existing account",
         "instance": "/api/v1/transactions/transfer",
         "trace_id": "8a250eaa650943b085934771fb35ba54",
         "timestamp": "2024-01-19T16:59:03.124+0000",
         "help": "please check if the destination account ID is correct or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information.",
         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
     },
   ]
 }
```
