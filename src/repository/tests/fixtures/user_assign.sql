INSERT INTO projects (name) VALUES ('Test Project 0');
INSERT INTO projects (name) VALUES ('Test Project 1');

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(1, 1, NULL, 0, 'Test PJ0 Major TASK', 'TEST PJ0 Major TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(2, 1, 1, 1, 'Test PJ0 Minor TASK', 'TEST PJ0 Minor TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(3, 1, 2, 2, 'Test PJ0 Trivial TASK', 'TEST PJ0 Trivial TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(4, 2, NULL, 0, 'Test PJ1 Major TASK', 'TEST PJ1 Major TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(5, 2, 4, 1, 'Test PJ1 Minor TASK', 'TEST PJ1 Minor TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(6, 2, 5, 2, 'Test PJ1 Trivial TASK NotStarted', 'TEST PJ1 Trivial TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(7, 2, 5, 2, 'Test PJ1 Trivial TASK InProgress', 'TEST PJ1 Trivial TASK - Description', 1, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(8, 2, 5, 2, 'Test PJ1 Trivial TASK Reviewing', 'TEST PJ1 Trivial TASK - Description', 2, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(9, 2, 5, 2, 'Test PJ1 Trivial TASK Cancelled', 'TEST PJ1 Trivial TASK - Description', 3, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(10, 2, 5, 2, 'Test PJ1 Trivial TASK Done', 'TEST PJ1 Trivial TASK - Description', 4, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(11, 2, 5, 2, 'Test PJ1 Trivial TASK DEADLINE 1000', 'TEST PJ1 Trivial TASK - Description', 4, 1000, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(12, 2, 5, 2, 'Test PJ1 Trivial TASK DEADLINE 10000', 'TEST PJ1 Trivial TASK - Description', 4, 10000, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(13, 2, 5, 2, 'Test PJ1 Trivial TASK CREATED_AT 1000', 'TEST PJ1 Trivial TASK - Description', 4, NULL, 1000, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(14, 2, 5, 2, 'Test PJ1 Trivial TASK CREATED_AT 10000', 'TEST PJ1 Trivial TASK - Description', 4, NULL, 10000, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(15, 2, 5, 2, 'Test PJ1 Trivial TASK UPDATED_AT 1000', 'TEST PJ1 Trivial TASK - Description', 4, NULL, 0, 1000);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(16, 2, 5, 2, 'Test PJ1 Trivial TASK UPDATED_AT 10000', 'TEST PJ1 Trivial TASK - Description', 4, NULL, 0, 10000);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(17, 2, 5, 2, 'Test PJ1 FULL OPT', 'TEST PJ1 FULL OPT - Description', 4, 99999, 99999, 99999);

INSERT INTO users (user_id, username, email, password_hash) VALUES (1, 'TestUser0', 'testuser0@example.com', 'password_hash_0');
INSERT INTO users (user_id, username, email, password_hash) VALUES (2, 'TestUser1', 'testuser1@example.com', 'password_hash_1');

INSERT INTO user_assign (user_id, task_id) VALUES (1, 3);
INSERT INTO user_assign (user_id, task_id) VALUES (2, 3);

INSERT INTO user_assign (user_id, task_id) VALUES (1, 11);
INSERT INTO user_assign (user_id, task_id) VALUES (2, 12);



