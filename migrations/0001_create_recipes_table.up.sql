CREATE TABLE recipes (
  id TEXT PRIMARY KEY NOT NULL,
  dish_name TEXT NOT NULL,
  ingredients TEXT NOT NULL,
  time_to_prepare TEXT NOT NULL,
  source TEXT NOT NULL
);

CREATE TABLE recipe_tags (
  recipe_id TEXT NOT NULL,
  tag TEXT NOT NULL,
  FOREIGN KEY (recipe_id) REFERENCES recipes(id)
);
