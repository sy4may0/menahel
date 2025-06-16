INSERT INTO projects (name) VALUES ('TestProject0');
INSERT INTO projects (name) VALUES ('TestProject1');

INSERT INTO users (username, email, password_hash) VALUES ('TestUser0', 'testuser0@example.com', 'password_hash_0');
INSERT INTO users (username, email, password_hash) VALUES ('TestUser1', 'testuser1@example.com', 'password_hash_1');
INSERT INTO users (username, email, password_hash) VALUES ('TestUser2', 'testuser2@example.com', 'password_hash_2');

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    1, 1, NULL, 0, 'Test_Major_Task0', 'Test_Major_Task0_Description', 0, 0, 0, NULL
);

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    2, 1, 1, 1, 'Test_Minor_Task0', 'Test_Minor_Task0_Description', 0, 0, 0, NULL
);

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    3, 1, 2, 2, 'Test_Trivial_Task0', 'Test_Trivial_Task0_Description', 0, 0, 0, NULL
);

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    4, 1, NULL, 0, 'Test_Major_Task1', 'Test_Major_Task1_Description', 0, 0, 0, NULL
);

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    5, 1, 4, 1, 'Test_Minor_Task1', 'Test_Minor_Task1_Description', 0, 0, 0, NULL
);

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    6, 1, 5, 2, 'Test_Trivial_Task1', 'Test_Trivial_Task1_Description', 0, 0, 0, NULL
);

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    7, 1, 5, 2, 'Test_Trivial_Task2', 'Test_Trivial_Task1_Description', 0, 0, 0, NULL
);

INSERT INTO tasks (
    task_id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
) VALUES (
    8, 1, 5, 2, 'Test_Trivial_Task3', 'Test_Trivial_Task1_Description', 0, 0, 0, NULL
);


INSERT INTO user_assign (
    user_assign_id, user_id, task_id
) VALUES (
    1, 1, 3
);

INSERT INTO user_assign (
    user_assign_id, user_id, task_id
) VALUES (
    2, 2, 3
);

INSERT INTO user_assign (
    user_assign_id, user_id, task_id
) VALUES (
    3, 3, 3
);

INSERT INTO user_assign (
    user_assign_id, user_id, task_id
) VALUES (
    4, 1, 6
);

INSERT INTO user_assign (
    user_assign_id, user_id, task_id
) VALUES (
    5, 1, 7
);

INSERT INTO user_assign (
    user_assign_id, user_id, task_id
) VALUES (
    6, 1, 8
);



