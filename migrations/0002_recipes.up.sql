ALTER TABLE recipes RENAME TO recipes_v1;

CREATE TABLE recipes (
    id VARCHAR(200) UNIQUE PRIMARY KEY NOT NULL,
    whos_there VARCHAR(200) NOT NULL,
    answer_who VARCHAR(200) NOT NULL,
    recipe_source VARCHAR(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS tags (
    recipe_id VARCHAR(200) NOT NULL,
    tag VARCHAR(200) NOT NULL,
    FOREIGN KEY (recipe_id)
    REFERENCES recipes(id)
);
