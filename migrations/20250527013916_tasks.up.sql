-- Add up migration script here
CREATE TABLE tasks (
    task_id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    parent_id INTEGER,
    level INTEGER NOT NULL CHECK (level BETWEEN 0 AND 2),
    name TEXT NOT NULL,
    description TEXT,
    status INTEGER NOT NULL,
    deadline INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER,
    FOREIGN KEY (project_id) REFERENCES projects(project_id),
    FOREIGN KEY (parent_id) REFERENCES tasks(task_id)
)