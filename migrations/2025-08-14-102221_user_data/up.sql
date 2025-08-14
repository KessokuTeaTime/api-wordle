CREATE TABLE user_data (
    id SERIAL PRIMARY KEY,
    session TEXT NOT NULL
);

CREATE TABLE user_data_entry (
    parent_id INT NOT NULL REFERENCES user_data(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    date TEXT NOT NULL
    status JSONB NOT NULL

    PRIMARY KEY (parent_id, date)
);
