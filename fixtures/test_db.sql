INSERT INTO users (id, username, email, password_hash) VALUES (0, 'testuser0', 'test0@example.com', 'dummy_hash_0');
INSERT INTO users (id, username, email, password_hash) VALUES (1, 'testuser1', 'test1@example.com', 'dummy_hash_1');
INSERT INTO users (id, username, email, password_hash) VALUES (2, 'testuser2', 'test2@example.com', 'dummy_hash_2');
INSERT INTO users (id, username, email, password_hash) VALUES (3, 'testuser3', 'test3@example.com', 'dummy_hash_3');
INSERT INTO users (id, username, email, password_hash) VALUES (4, 'testuser4', 'test4@example.com', 'dummy_hash_4');
INSERT INTO users (id, username, email, password_hash) VALUES (5, 'testuser5', 'test5@example.com', 'dummy_hash_5');
INSERT INTO users (id, username, email, password_hash) VALUES (6, 'testuser6', 'test6@example.com', 'dummy_hash_6');
INSERT INTO users (id, username, email, password_hash) VALUES (7, 'testuser7', 'test7@example.com', 'dummy_hash_7');
INSERT INTO users (id, username, email, password_hash) VALUES (8, 'testuser8', 'test8@example.com', 'dummy_hash_8');
INSERT INTO users (id, username, email, password_hash) VALUES (9, 'testuser9', 'test9@example.com', 'dummy_hash_9');
INSERT INTO projects (id, name) VALUES (0, 'TestProject0');
INSERT INTO projects (id, name) VALUES (1, 'TestProject1');
INSERT INTO projects (id, name) VALUES (2, 'TestProject2');
INSERT INTO projects (id, name) VALUES (3, 'TestProject3');
INSERT INTO projects (id, name) VALUES (4, 'TestProject4');
INSERT INTO projects (id, name) VALUES (5, 'TestProject5');
INSERT INTO projects (id, name) VALUES (6, 'TestProject6');
INSERT INTO projects (id, name) VALUES (7, 'TestProject7');
INSERT INTO projects (id, name) VALUES (8, 'TestProject8');
INSERT INTO projects (id, name) VALUES (9, 'TestProject9');

INSERT INTO tasks 
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (0, 'TestMajorTask0', 'TestTask0Description', 0, 0, 0, NULL, 1000, 1000, 1000);

INSERT INTO tasks 
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (1, 'TestMinorTask1', 'TestTask1Description', 1, 0, 0, 1, 1000, 2000, 2000);

INSERT INTO tasks 
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (2, 'TestTrivialTask2', 'TestTask2Description', 2, 0, 0, 2, 1000, 2000, 2000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (3, 'TestNotStartedTask3', 'TestTask3Description', 2, 1, 0, 2, 1500, 3000, 3000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (4, 'TestInProgressTask4', 'TestTask4Description', 2, 1, 0, 2, 1500, 3000, 3000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (5, 'TestReviewingTask5', 'TestTask5Description', 2, 2, 0, 2, 2000, 3000, 3000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (6, 'TestCancelledTask6', 'TestTask6Description', 2, 3, 0, 2, 1000000000, 1000000000, 1000000000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (7, 'TestDoneTask7', 'TestTask7Description', 2, 4, 0, 2, 1000000000, 1000000000, 1000000000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (8, 'TestMajorTaskProject2', 'TestTask8Description', 0, 0, 2, 0, 1000000000, 1000000000, 1000000000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (9, 'TestMinorTaskProject2', 'TestTask9Description', 1, 0, 2, 0, 1000000000, 1000000000, 1000000000);

INSERT INTO tasks
    (id, name, description, level, status, project_id, parent_id, deadline, created_at, updated_at)
VALUES 
    (10, 'TestTrivialTaskProject2', 'TestTask10Description', 2, 0, 2, 0, 1000000000, 1000000000, 1000000000);