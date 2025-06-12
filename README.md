# Recipe Server

**Author:** Tommy Trakoolthai
**Date:** 2025-06

This web application is a recipe management service built entirely in Rust using a modern asynchronous web stack. It leverages **Tokio** for async runtime, **Axum** for routing and HTTP handling, **Askama** for server-side HTML templating, **SQLx** for compile-time verified SQL queries, and **SQLite** for lightweight persistent storage. The service includes:

* A clean HTML-based frontend powered by Askama templates
* A fully documented REST API using **Utoipa** and **Swagger UI**
* A WASM-based browser client (e.g., using **Leptos**) for interacting with the API
* JWT-based authentication for secure operations

---

## Build and Run

By default, the database URI is:

```
sqlite://db/recipes.db
```

If the database does **not** already exist, create it manually with:

```bash
mkdir -p db
sqlite3 db/recipes.db '.tables'
```

You can override this URI using the `DATABASE_URL` environment variable or the `--db-uri` command-line argument.

To build and run the project for the first time and populate it with sample recipes:

```bash
cargo run --release -- --init-from assets/static/recipes.json
```

---

## Development Notes

Install the SQLx CLI tool if you haven't:

```bash
cargo install sqlx-cli
```

To add a new migration (in reverse sequential order):

```bash
sqlx migrate add -r -s <name>
```

To run all migrations:

```bash
sqlx migrate run
```

To update compile-time SQLx query checks:

```bash
DATABASE_URL=sqlite://db/recipes.db cargo sqlx prepare
```

Before committing, include SQLx metadata and migrations:

```bash
git add .sqlx migrations
```

---

## Included Files for Grading

The `db/recipes.db` file and `secrets/` directory are included in this repository for **ease of grading**. This ensures the project runs immediately without extra setup.

> ⚠️ In a production or industry setting, these would be excluded via `.gitignore` and configured using environment variables or external secret management.

If omitted, the program will automatically attempt to create the database at `db/recipes.db`, and the user must supply their own `secrets/` directory containing:

* `jwt_secret.txt`
* `reg_password.txt`

---

## Known Issues / Future Work

* The `main.rs` file is currently too large and monolithic. Future versions will modularize it into smaller, more maintainable components.
* A working Dockerfile is planned but has not yet been finalized or included in this version.

---

## License

This project is licensed under **Apache 2.0 OR MIT**. See `LICENSE.txt` for full details.
