# Recipe Server

**Author:** Tommy Trakoolthai\
**Date:** 2025-06

This web application is a recipe management service built entirely in Rust using a modern async web stack. It leverages Tokio for asynchronous runtime, Axum for routing and HTTP handling, Askama for server-side HTML templating, SQLx for compile-time checked SQL, and SQLite for lightweight persistent storage. The service provides:

- A clean, HTML-based frontend interface powered by Askama templates.
- A fully documented REST API with Utoipa and Swagger UI, exposing CRUD endpoints.
- A WASM-based browser client (Leptos) for interacting with the API.
- JWT-based authentication for protected operations.

⸻

## Build and Run

By default, the database URI is:

```
sqlite://db/recipes.db
```

If the database does not already exist, then run:

```bash
mkdir -p db
sqlite3 db/recipes.db '.tables'
```

You can override this URI with either the `DATABASE_URL` environment variable or the `--db-uri` command-line argument.

To build and run the project for the first time and load an initial set of recipes, use:

```bash
cargo run --release -- --init-from assets/static/recipes.json
```

This will populate a newly-created database with sample recipes.

⸻

## Leptos Web App Mode vs Static Server

This project supports two frontend UIs:

1. **Static Server Mode** using Askama templates rendered on the server.
2. **Leptos Web App Mode** using a WASM client application served at `/ui`.

In Leptos mode, we serve the WASM frontend using a one-liner command that runs both the Leptos compiler and backend server:

```bash
trunk serve & cargo run
```

This enables fast iteration on the Leptos app while developing against the running Axum backend.

⸻

## Development Notes

Install the SQLx CLI tool if you haven’t:

```bash
cargo install sqlx-cli
```

Migrations are enabled with reverse sequential order. To add a new migration:

```bash
sqlx migrate add -r -s <name>
```

To apply the latest migrations to your database:

```bash
sqlx migrate run
```

Compile-time checking of SQL queries is enabled. If you update schema or queries, update `.sqlx` metadata with:

```bash
DATABASE_URL=sqlite://db/recipes.db cargo sqlx prepare
```

Remember to add migration artifacts and SQLx metadata before committing:

```bash
git add .sqlx migrations
```

⸻

## Included Files for Grading

The `db/recipes.db` and `secrets/` directory are intentionally included in the repository to make grading easier and ensure the project runs immediately. In a production or industry setting, both would typically be excluded and managed via `.gitignore` or environment-based configuration. In this case, the program would create and initialize the database at `db/recipes.db`, and the user would need to configure their own `/secrets` directory containing `jwt_secret.txt` and `reg_password.txt`.

⸻

## ⚠ Known Issues / Future Work

- The project would benefit from additional modularization, especially in `recipe-server/main.rs`, which currently holds too much logic in one place.
- A working Dockerfile is planned but not yet fully implemented.

⸻

## Attribution

Much of the code, along with the above instructions were taken and adapted from Bart Massey. Source code can be found at https://github.com/pdx-cs-rust-web/knock-knock-2.

ChatGPT was used in the generation of recipes.json and during the development process.

⸻

## 📄 License

This project is licensed under **Apache 2.0 OR MIT**. See `LICENSE.txt` for details.
