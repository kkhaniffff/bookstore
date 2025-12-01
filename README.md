# 📚 Bookstore 

A small learning playground built while exploring Rust and the Axum framework.
The project focuses on understanding how to structure a backend service with authentication, a PostgreSQL database, and a Docker based workflow.
It aims to provide a clear and approachable environment for experimenting with Rust backend development and modern tooling.

---

## ✨ Overview

This project provides a minimal yet realistic backend environment. It focuses on:

* **Axum** for clean, modular request handling
* **SQLx** for async PostgreSQL access with compile time safety
* **Docker Compose** for consistent development environments
* **Automatic SQL migrations** applied at startup
* **JWT based authentication** with user and admin roles

---

## 🛠 Requirements

You can run the project either with Docker or directly on your local machine.

### Docker setup

* Docker
* Docker Compose

### Local setup

* Rust stable
* PostgreSQL
* `sqlx-cli` for managing migrations

---

## 🚀 Getting Started

### 1. Configure environment variables

Duplicate the example file and adjust values as needed.

```
cp .env.example .env
```

Set database credentials and JWT secrets here.
Refer to `.env.example` for all available fields.

### 2. Start the application

#### Option 1: Using Docker

Build and launch the API and database together.

```
docker compose up --build
```

#### Option 2: Running locally

Start PostgreSQL on your machine, then apply migrations.

```
sqlx migrate run
```

Run the application:

```
cargo run
```

### 3. Once running, the API is available at:

```
http://localhost:3000
```

All available API endpoints are documented in the `app.http` file.

---

## 🧩 SQLx Offline Mode

This project uses SQLx in offline mode so query metadata is stored locally. This allows the code to compile without needing a live database connection.

When you make changes to the database schema, run:

```
sqlx migrate run
sqlx migrate prepare
```

Be sure to commit the generated SQLx files.

---

## 📄 License

This project is distributed under the MIT License.

## 🤝 Contributions

Suggestions and improvements are welcome.
