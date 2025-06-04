-- Add up migration script here
CREATE TABLE IF NOT EXISTS recipes (
    whos_there VARCHAR(200) NOT NULL,
    answer_who VARCHAR(200) NOT NULL
)
