# Recipe Server

Tommy Trakoolthai
CS510: Rust Web Full-Stack

# Description

This project uses a Axum, Askama, Sqlx, Sqlite, and Tokio stack to serve food recipes. Much of the code and the
following instructions are provided by Bart Massey in the repository:
https://github.com/pdx-cs-rust-web/knock-knock-2.

# Build and Run

By default the `recipe` database URI is
`sqlite://db/recipes.db`. You can override this with the
`RECIPES_DB_URI` environment variable or with the `--db-uri`
command-line argument.

To build and run this code for the first time, an initial collection of recipes
should be loaded into a newly-create database. This can be done by running
the following command:

    cargo run --release -- --init-from assets/static/recipes.json

# Development

For working on the code, run the following command:

    cargo install sqlx-cli`

Ensure that the environment variable is set for the database:

        export DATABASE_URL=sqlite://db/recipes.db

* `sqlx` migrations are turned on, with reverse
  sequential migrations. Add a migration called `<name>` with

        sqlx migrate add -r -s <name>

  and then edit the migration files.

* `sqlx` compile-time checking of queries against
  the database schemas is turned on. If you modify the
  database schemas or the queries in the source code, please
  run

        sqlx prepare

  to update things so that users can compile before the
  database is built and migrated.

Because of the above you may need to

    git add .sqlx migrations

before committing to ensure things are up to date.

# Attrition

Much of the code and the above instructions were taken from Bart Massey; ChatGPT was used to generate recipes.json

# License
This work is made available under the "MIT License". See the file LICENSE.txt in this distribution for license terms.
