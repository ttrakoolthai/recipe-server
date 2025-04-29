-- https://www.shuttle.rs/blog/2023/10/04/sql-in-rust

CREATE TABLE recipes (
    id TEXT PRIMARY KEY,
    dish_name TEXT NOT NULL,
    ingredients TEXT NOT NULL,
    time_to_prepare TEXT NOT NULL,
    source TEXT NOT NULL
);
