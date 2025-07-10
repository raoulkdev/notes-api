# Notes API

A simple, clean REST API built with **Rust** and **Axum** for creating, retrieving, and deleting text notes. This project showcases best practices for designing APIs in Rust — including modular architecture, shared in-memory state, input validation, and structured JSON responses.

---

## Features

- Create new notes via `POST /notes`
- Retrieve all notes via `GET /notes`
- Get a single note by ID via `GET /notes/{id}`
- Delete a note by ID via `DELETE /notes/{id}`
- Input validation (empty titles are rejected)
- Auto-incrementing note IDs
- JSON request/response format
- Modular, extensible codebase

---

## Tech Stack

| Tech          | Purpose                                |
|---------------|----------------------------------------|
| **Rust**      | Core programming language              |
| **Axum**      | Web framework for building APIs        |
| **Tokio**     | Asynchronous runtime                   |
| **Serde**     | JSON serialization and deserialization |


---

## Example API Usage

### Create a Note

**POST** `/notes`  
```json
{
  "title": "Grocery List",
  "body": "Milk, eggs, bread, chicken, spinach, coffee"
}
```

**Response** `201 Created`  
```json
{
  "id": 1,
  "title": "Grocery List",
  "body": "Milk, eggs, bread, chicken, spinach, coffee"
}
```

---

### Get All Notes

**GET** `/notes`  
**Response** `200 OK`  
```json
[
  {
    "id": 1,
    "title": "Grocery List",
    "body": "Milk, eggs, bread, chicken, spinach, coffee"
  }
]
```

---

### Get Note by ID

**GET** `/notes/1`  
**Response** `302 Found`  
```json
{
  "id": 1,
  "title": "Grocery List",
  "body": "Milk, eggs, bread, chicken, spinach, coffee"
}
```

---

### Delete Note by ID

**DELETE** `/notes/1`  
**Response** `204 No Content`

---

## Sample Notes to Test With

You can use the following payloads in Postman or curl to test:

```json
{ "title": "Grocery List", "body": "Milk, eggs, bread, chicken, spinach, coffee." }
{ "title": "Workout Plan - Tuesday", "body": "Deadlifts, pull-ups, overhead press, core work." }
{ "title": "Meeting Notes - July 9", "body": "Discussed Q3 goals, sprint blockers, and deployment plan." }
{ "title": "Daily Journal - July 8", "body": "Learned Axum, went for a walk, felt productive." }
{ "title": "Study Topics for Finals", "body": "Rust ownership, HTTP status codes, SQL joins, async JS." }
```

---

## How to Run Locally

1. **Clone the repository**  
   ```bash
   git clone https://github.com/raoulkdev/notes-api.git
   cd notes-api
   ```

2. **Run the server**  
   ```bash
   cargo run
   ```

3. Server will start at:  
   ```
   http://localhost:3000
   ```

---

## 📚 Why This Project?

This project was built to showcase:
- Practical use of Axum for RESTful APIs
- Safe concurrent state handling with `Arc<Mutex<T>>`
- Clear code structure and scalable API design

---

## 📇 License

This project is open-source and available under the MIT License.
