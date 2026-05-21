# Assignment 12 OpenAPI Documentation

## Swagger UI

Swagger UI available at:

```text
http://127.0.0.1:3000/docs
```

---

## API Endpoints

### GET /api/books

Returns all books.

### POST /api/books

Creates a new book.

Example JSON:

```json
{
  "book_id": "1",
  "title": "Rust API",
  "isbn": "ISBN100",
  "status": "Available"
}
```

---

## Error Handling

Possible responses:
- 200 OK
- 404 Not Found
- 500 Internal Server Error

---

## Screenshot Evidence

```text
screenshots/swagger-ui.png
```
