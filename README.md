# 🦀 Axum Mongo API Boilerplate
[![CI](https://github.com/Rishalkp367/axum-mongo-app/actions/workflows/ci.yml/badge.svg)](https://github.com/Rishalkp367/axum-mongo-app/actions)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](#)
[![Docker](https://img.shields.io/badge/docker-ready-blue)](#)
[![MongoDB](https://img.shields.io/badge/mongodb-7-green)](#)
[![License](https://img.shields.io/badge/license-MIT-purple)](#)

A production-ready Rust backend starter built with **Axum**, **MongoDB**, and **Docker** — focused on clean architecture, scalability, and real-world practices.

---

## ✨ Tech Stack

- ⚡ Axum (async Rust web framework)
- 🧵 Tokio multithreaded runtime
- 🍃 MongoDB official async driver
- 🐳 Docker & Docker Compose
- 📚 Swagger / OpenAPI (utoipa)
- 📈 Tracing & structured logging

---

## 🚀 Features

- Clean layered architecture (routes, repositories, models)
- MongoDB connection pooling
- Repository pattern
- Health checks (liveness & readiness)
- Swagger API documentation
- Dockerized local development
- Graceful shutdown (SIGINT + SIGTERM)
- Production-friendly configuration via `.env`

---

## 📁 Project Structure

```
src/
├── app_state.rs
├── config.rs
├── db.rs
├── docs.rs
├── models/
├── repositories/
├── routes/
└── main.rs
```

---

## 🧪 Health Endpoints

| Endpoint | Description |
|---------|-------------|
| `/health/live` | Service liveness |
| `/health/ready` | MongoDB readiness |

---

## 📚 API Documentation

Swagger UI available at:

```
http://localhost:3000/docs
```

---

## 🐳 Run with Docker (Recommended)

```bash
docker compose up --build
```

API will be available at:

```
http://localhost:3000
```

---

## 💻 Run Locally (without Docker)

Make sure MongoDB is running locally.

```bash
cargo run
```

---

## 📦 Example Usage

Create user:

```bash
curl -X POST http://localhost:3000/users \
-H "Content-Type: application/json" \
-d '{"name":"Rishal","email":"rishal@test.com"}'
```

Fetch users:

```bash
curl http://localhost:3000/users
```

---

## 🛑 Graceful Shutdown

Supports:

- Ctrl + C
- SIGTERM (Docker/Kubernetes compatible)

---

## 🧱 Architecture Philosophy

- Separation of concerns
- Explicit module boundaries
- Repository pattern
- Production-first design

---

## 🧭 Roadmap

- JWT authentication
- Rate limiting
- Pagination
- Centralized error handling
- API versioning
- Role-based access control

---

## 📜 License

MIT

---

### ⭐ If you find this useful, consider starring the repo!
