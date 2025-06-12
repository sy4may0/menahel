INSERT INTO users (user_id, username, email, password_hash) VALUES (1, 'TestUser0', 'test0@example.com', 'password0');
INSERT INTO users (user_id, username, email, password_hash) VALUES (2, 'TestUser1', 'test1@example.com', 'password1');
INSERT INTO users (user_id, username, email, password_hash) VALUES (3, 'TestUser2', 'test2@example.com', 'password2');
INSERT INTO users (user_id, username, email, password_hash) VALUES (4, 'TestUser3', 'test3@example.com', 'password3');
INSERT INTO users (user_id, username, email, password_hash) VALUES (5, 'TestUser4', 'test4@example.com', 'password4');
INSERT INTO users (user_id, username, email, password_hash) VALUES (6, 'TestUser5', 'test5@example.com', 'password5');
INSERT INTO users (user_id, username, email, password_hash) VALUES (7, 'TestUser6', 'test6@example.com', 'password6');
INSERT INTO users (user_id, username, email, password_hash) VALUES (8, 'TestUser7', 'test7@example.com', 'password7');
INSERT INTO users (user_id, username, email, password_hash) VALUES (9, 'TestUser8', 'test8@example.com', 'password8');
INSERT INTO users (user_id, username, email, password_hash) VALUES (10, 'TestUser9', 'test9@example.com', 'password9');

INSERT INTO projects (project_id, name) VALUES (1, 'TestProject0');

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(1, 1, NULL, 0, 'Test PJ0 Major TASK', 'TEST PJ0 Major TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(2, 1, 1, 1, 'Test PJ0 Minor TASK', 'TEST PJ0 Minor TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(3, 1, 2, 2, 'Test PJ0 Trivial TASK', 'TEST PJ0 Trivial TASK - Description', 0, 0, 0, NULL);

INSERT INTO tasks (task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at) VALUES
(4, 1, 2, 2, 'Test PJ0 Trivial TASK 2', 'TEST PJ0 Trivial TASK 2 - Description', 0, 0, 0, NULL);


INSERT INTO user_assign (user_id, task_id) VALUES (1, 3);
INSERT INTO user_assign (user_id, task_id) VALUES (2, 3);
INSERT INTO user_assign (user_id, task_id) VALUES (2, 4);