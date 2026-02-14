# 🦀 axum-mongo-app

[![CI](https://github.com/Rishalkp367/axum-mongo-app/actions/workflows/ci.yml/badge.svg)](https://github.com/Rishalkp367/axum-mongo-app/actions)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](#)
[![Docker](https://img.shields.io/badge/docker-ready-blue)](#)
[![MongoDB](https://img.shields.io/badge/mongodb-7-green)](#)
[![License](https://img.shields.io/badge/license-MIT-purple)](#)

Production-ready Rust backend boilerplate using:

- Axum 0.7
- Tokio (multithreaded runtime)
- MongoDB (official async driver)
- Docker + Docker Compose
- Health checks (liveness + readiness)
- Repository pattern
- Graceful shutdown

---

## 🚀 Features

- Multithreaded async runtime
- MongoDB connection pooling
- Clean modular architecture
- Dockerized environment
- Health endpoints
- Example CRUD (Users)
- Graceful shutdown (SIGINT + SIGTERM)
- Production-ready configuration via `.env`

---

## 📁 Project Structure

src/
├── config.rs
├── db.rs
├── app_state.rs
├── models/
│ └── user_model.rs
├── repositories/
│ └── user_repository.rs
├── routes/
│ ├── health.rs
│ ├── users.rs
│ └── mod.rs

---

## 🧪 Health Endpoints

| Endpoint        | Purpose                 |
| --------------- | ----------------------- |
| `/health/live`  | Liveness check          |
| `/health/ready` | MongoDB readiness check |

---

## 🐳 Run With Docker

```bash
docker-compose up --build

API runs at:

http://localhost:3000


Test health:

http://localhost:3000/health/live
```

---

## 🧑‍💻 Run Locally (Without Docker)

Make sure MongoDB is running locally.

```bash

cargo run

```

---

## 📦 Example CRUD

Create user:

```bash
curl -X POST http://localhost:3000/users \
-H "Content-Type: application/json" \
-d '{"name":"Rishal","email":"rishal@test.com"}'


List users:

curl http://localhost:3000/users

```

---

## 📦 Example CRUD

Create user:

```bash
curl -X POST http://localhost:3000/users \
-H "Content-Type: application/json" \
-d '{"name":"Rishal","email":"rishal@test.com"}'


List users:

curl http://localhost:3000/users
```

---

## 🛑 Graceful Shutdown

Supports:

Ctrl+C

SIGTERM (Docker / Kubernetes)

---

## 🧱 Architecture Philosophy

Separation of concerns

Repository pattern

Explicit module boundaries

Production-first design

---

## 🧭 Roadmap

Centralized error middleware

JWT authentication

Role-based access control

API versioning

OpenAPI / Swagger

Rate limiting

---

## 📜 License

MIT

---

### ✅ How to use it

1. Create file:

```bash
touch README.md
```

2. Open it and paste everything from the block above.

3. Save.

---
