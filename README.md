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