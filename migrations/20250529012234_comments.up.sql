-- Add up migration script here
CREATE TABLE comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (task_id) REFERENCES tasks (id)
);
