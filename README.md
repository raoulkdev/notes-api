# Notes API
[![Last Commit](https://img.shields.io/github/last-commit/raoulkdev/notes-api?style=flat&logo=git)](https://github.com/raoulkdev/notes-api/commits)
[![Top Language](https://img.shields.io/github/languages/top/raoulkdev/notes-api?style=flat&logo=c%2B%2B)](https://github.com/raoulkdev/notes-api)
[![Contributors](https://img.shields.io/github/contributors/raoulkdev/notes-api?style=flat&logo=github)](https://github.com/raoulkdev/notes-api/graphs/contributors)

> A simple, clean REST API built with **Rust** and **Axum** for creating, retrieving, and deleting text notes that uses **PostgreSQL** for persistent data storage. This project includes modular architecture, input validation, structured JSON responses, and database integration.

[Read my article about making this](https://medium.com/@nkumba/building-a-clean-notes-api-in-rust-using-axum-0bd888578c81)

[Try the API](https://notes-api-wfeh.onrender.com/notes)

---

## Features

- Create new notes via `POST /notes`
- Retrieve all notes via `GET /notes`
- Get a single note by ID via `GET /notes/{id}`
- Delete a note by ID via `DELETE /notes/{id}`
- Input validation (empty titles are rejected)
- Randomy generated note UUIDs with database-backed storage
- JSON request/response format
- Modular, extensible codebase
- **Persistent storage with PostgreSQL**

---

## Tech Stack

| Tech             | Purpose                                     |
|------------------|---------------------------------------------|
| **Rust**         | Core programming language                   |
| **Axum**         | Web framework for building APIs             |
| **Tokio**        | Asynchronous runtime                        |
| **Serde**        | JSON serialization and deserialization      |
| **SQLx**         | Async database driver for PostgreSQL        |
| **PostgreSQL**   | Relational data storage                     |

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
    "id": "24cdcbd6-f64b-4184-8316-073506447554",
    "title": "Grocery List",
    "body": "Milk, eggs, bread, chicken, spinach, coffee",
    "created_time": "2025-07-12T22:02:53.026158"
}
```

---

### Get All Notes

**GET** `/notes`  
**Response** `200 OK`  
```json
[
  {
    "id": "24cdcbd6-f64b-4184-8316-073506447554",
    "title": "Grocery List",
    "body": "Milk, eggs, bread, chicken, spinach, coffee",
    "created_time": "2025-07-12T22:02:53.026158"
  }
]
```

---

### Get Note by ID

**GET** `/notes/24cdcbd6-f64b-4184-8316-073506447554`  
**Response** `200 OK`  
```json
{
    "id": "24cdcbd6-f64b-4184-8316-073506447554",
    "title": "Grocery List",
    "body": "Milk, eggs, bread, chicken, spinach, coffee",
    "created_time": "2025-07-12T22:02:53.026158"
}
```

---

### Delete Note by ID

**DELETE** `/notes/24cdcbd6-f64b-4184-8316-073506447554`  
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

2. **Set up PostgreSQL**  
   - Install PostgreSQL locally or use a cloud provider (e.g., [Supabase](https://supabase.com/) or [Neon](https://neon.tech/)).
   - Create a database, e.g. `notes_api`.
   - Create a `.env` file in the project root and add your database URL:
     ```
     DATABASE_URL=postgres://username:password@localhost:5432/notes_api
     ```
   - Run the migrations if provided (see any `migrations` folder or SQL script).

3. **Run the server**  
   ```bash
   cargo run
   ```

4. Server will start at:  
   ```
   http://localhost:3000
   ```

---

## Why This Project?

This project was built to showcase:
- Making RESTful APIs with Rust and Axum
- Safe async database handling with SQLx and PostgreSQL
- Modular, scalable code structure
- Real-world input validation and error handling

---

## License

This project is open-source and available under the MIT License.
