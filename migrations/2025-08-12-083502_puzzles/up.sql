CREATE TABLE puzzles (
    date TEXT PRIMARY KEY,
    puzzle TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
)
