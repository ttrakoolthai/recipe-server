ALTER TABLE recipes RENAME TO recipes_v1;

CREATE TABLE recipes (
  id VARCHAR(200) UNIQUE PRIMARY KEY NOT NULL,
  dish_name VARCHAR(200) NOT NULL,
  ingredients VARCHAR(200) NOT NULL,
  time_to_prepare VARCHAR(200) NOT NULL,
  source VARCHAR(200) NOT NULL
);

CREATE TABLE tags (
  recipe_id TEXT NOT NULL,
  tag TEXT NOT NULL,
  FOREIGN KEY (recipe_id) REFERENCES recipes(id)
);
