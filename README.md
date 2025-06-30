# Explore API

This is a simple API built with Axum in Rust.

## Endpoints

### GET /

Returns a simple "Hello, World!" message.

**Example Request:**

```bash
curl http://localhost:3000/
```

**Example Response:**

```
Hello, World!
```

### POST /

Creates a new user.

**Example Request:**

```bash
curl -X POST http://localhost:3000/ -H "Content-Type: application/json" -d '{"name": "John Doe"}'
```

**Example Response:**

```json
{
  "id": 1337,
  "name": "John Doe"
}
```

### GET /users

Lists all users in the database.

**Example Request:**

```bash
curl http://localhost:3000/users
```

**Example Response:**

```json
[
  {
    "id": 12345,
    "name": "John Doe"
  },
  {
    "id": 67890,
    "name": "Jane Smith"
  }
]
```

### GET /users/:id

Retrieves details of a single user by their ID.

**Example Request:**

```bash
curl http://localhost:3000/users/12345
```

**Example Response (Success):**

```json
{
  "id": 12345,
  "name": "John Doe"
}
```

**Example Response (Not Found):**

```
HTTP/1.1 404 Not Found
content-length: 0
date: ...
```

### PUT /users/:id

Updates an existing user by their ID.

**Example Request:**

```bash
curl -X PUT http://localhost:3000/users/12345 -H "Content-Type: application/json" -d '{"name": "Jane Doe"}'
```

**Example Response (Success):**

```json
{
  "id": 12345,
  "name": "Jane Doe"
}
```

**Example Response (Not Found):**

```
HTTP/1.1 404 Not Found
content-length: 0
date: ...
```

### DELETE /users/:id

Deletes a user by their ID.

**Example Request:**

```bash
curl -X DELETE http://localhost:3000/users/12345
```

**Example Response (Success):**

```
HTTP/1.1 204 No Content
content-length: 0
date: ...
```

**Example Response (Not Found):**

```
HTTP/1.1 404 Not Found
content-length: 0
date: ...
```