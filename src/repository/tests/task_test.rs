use crate::enums::TaskLevel;
use crate::enums::TaskStatus;
use crate::models::{Task, task::TaskFilter};
use crate::repository::task_repo::{TaskRepository, get_task_by_id_with_transaction};
use chrono::Utc;
use sqlx::sqlite::SqlitePool;

#[cfg(test)]
mod task_repo_test {
    use super::*;

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_create_task(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let now = Utc::now().timestamp();

        let task = Task::new(
            1,
            None,
            TaskLevel::Major.to_int(),
            "Test Task".to_string(),
            Some("Test Task Description".to_string()),
            TaskStatus::NotStarted.to_int(),
            None,
        );
        let created_task = task_repo.create_task(task).await.unwrap();

        let retrieved_task = task_repo
            .get_task_by_id(created_task.id.unwrap())
            .await
            .unwrap();

        assert_eq!(retrieved_task.project_id, 1);
        assert!(retrieved_task.parent_id.is_none());
        assert_eq!(retrieved_task.level, TaskLevel::Major.to_int());
        assert_eq!(retrieved_task.name, "Test Task");
        assert_eq!(
            retrieved_task.description,
            Some("Test Task Description".to_string())
        );
        assert_eq!(retrieved_task.status, TaskStatus::NotStarted.to_int());
        assert!(retrieved_task.deadline.is_none());
        assert!(retrieved_task.created_at >= now);
        assert!(retrieved_task.updated_at.is_some());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_create_task_with_parent_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let now = Utc::now().timestamp();

        let task = Task::new(
            1,
            Some(1),
            TaskLevel::Minor.to_int(),
            "Test Task with Parent".to_string(),
            Some("Test Task with Parent Description".to_string()),
            TaskStatus::NotStarted.to_int(),
            None,
        );
        let created_task = task_repo.create_task(task).await.unwrap();
        assert_eq!(created_task.id, Some(18));
        assert_eq!(created_task.project_id, 1);
        assert_eq!(created_task.parent_id, Some(1));
        assert_eq!(created_task.level, TaskLevel::Minor.to_int());
        assert_eq!(created_task.name, "Test Task with Parent");
        assert_eq!(
            created_task.description,
            Some("Test Task with Parent Description".to_string())
        );
        assert_eq!(created_task.status, TaskStatus::NotStarted.to_int());
        assert!(created_task.deadline.is_none());
        assert!(created_task.created_at >= now);
        assert!(created_task.updated_at.is_some());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_task_by_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = task_repo.get_task_by_id(1).await.unwrap();
        assert_eq!(task.id, Some(1));
        assert_eq!(task.project_id, 1);
        assert!(task.parent_id.is_none());
        assert_eq!(task.level, TaskLevel::Major.to_int());
        assert_eq!(task.name, "Test PJ0 Major TASK");
        assert_eq!(
            task.description,
            Some("TEST PJ0 Major TASK - Description".to_string())
        );
        assert_eq!(task.status, TaskStatus::NotStarted.to_int());
        assert!(task.deadline.unwrap() == 0);
        assert!(task.created_at == 0);
        assert!(task.updated_at.is_none());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_task_by_id_not_exists(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = task_repo.get_task_by_id(100).await;
        assert!(task.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_all_tasks(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let tasks = task_repo.get_all_tasks().await.unwrap();
        assert_eq!(tasks.len(), 17);
        assert_eq!(tasks[15].id, Some(16));
        assert_eq!(tasks[15].project_id, 2);
        assert_eq!(tasks[15].parent_id, Some(5));
        assert_eq!(tasks[15].level, TaskLevel::Trivial.to_int());
        assert_eq!(tasks[15].name, "Test PJ1 Trivial TASK UPDATED_AT 10000");
        assert_eq!(
            tasks[15].description,
            Some("TEST PJ1 Trivial TASK - Description".to_string())
        );
        assert_eq!(tasks[15].status, TaskStatus::Done.to_int());
        assert!(tasks[15].deadline.is_none());
        assert!(tasks[15].created_at == 0);
        assert!(tasks[15].updated_at.unwrap() == 10000);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_no_filter(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let tasks = task_repo.get_tasks_by_filter(None, None, None).await.unwrap();
        assert_eq!(tasks.len(), 17);
    }


    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_no_filter_page_and_page_size(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let tasks = task_repo.get_tasks_by_filter(None, Some(&1), Some(&10)).await.unwrap();
        assert_eq!(tasks.len(), 10);
    }


    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_and_pagination(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: Some(2),
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), Some(&3), Some(&5)).await.unwrap();
        assert_eq!(tasks.len(), 4);
        for task in tasks {
            assert_eq!(task.project_id, 2);
        }
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_invalid_no_pagination(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let tasks = task_repo.get_tasks_by_filter(None, None, Some(&5)).await;
        assert!(tasks.is_err());

        let tasks = task_repo.get_tasks_by_filter(None, Some(&1), None).await;
        assert!(tasks.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_invalid_pagination(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let tasks = task_repo.get_tasks_by_filter(None, Some(&-1), Some(&5)).await;
        assert!(tasks.is_err());

        let tasks = task_repo.get_tasks_by_filter(None, Some(&0), Some(&0)).await;
        assert!(tasks.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_long_page_size(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let tasks = task_repo.get_tasks_by_filter(None, Some(&1), Some(&101)).await;
        assert!(tasks.is_err());
    }
    
    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_project_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: Some(1),
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 3);
        for task in tasks {
            assert_eq!(task.project_id, 1);
        }
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_parent_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: Some(1),
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].parent_id, Some(1));
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_level(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: Some(TaskLevel::Minor.to_int()),
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };

        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 2);
        for task in tasks {
            assert_eq!(task.level, TaskLevel::Minor.to_int());
        }
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_name(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: Some("PJ0 Trivial".to_string()),
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "Test PJ0 Trivial TASK");
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_description(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: Some("PJ0 Trivial TASK".to_string()),
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(
            tasks[0].description,
            Some("TEST PJ0 Trivial TASK - Description".to_string())
        );
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_status(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: Some(TaskStatus::NotStarted.to_int()),
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 6);
        for task in tasks {
            assert_eq!(task.status, TaskStatus::NotStarted.to_int());
        }
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_deadline(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: Some(999),
            deadline_to: Some(1000),
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        for task in tasks {
            assert!(task.deadline.is_some());
            assert!(task.deadline.unwrap() >= 999);
            assert!(task.deadline.unwrap() <= 1000);
        }

        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: Some(8000),
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 2);
        for task in tasks {
            assert!(task.deadline.is_some());
            assert!(task.deadline.unwrap() >= 8000);
        }

        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: Some(3000),
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 11);
        for task in tasks {
            assert!(task.deadline.is_some());
            assert!(task.deadline.unwrap() <= 3000);
        }
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_created_at(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: Some(999),
            created_at_to: Some(1000),
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].created_at == 1000);

        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: Some(8000),
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 2);
        for task in tasks {
            assert!(task.created_at >= 8000);
        }

        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: Some(3000),
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 15);
        for task in tasks {
            assert!(task.created_at <= 3000);
        }
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_updated_at(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: Some(999),
            updated_at_to: Some(1000),
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].updated_at.unwrap() == 1000);

        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: Some(8000),
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 2);
        for task in tasks {
            assert!(task.updated_at.unwrap() >= 8000);
        }

        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: Some(3000),
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].updated_at.unwrap() <= 3000);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_multiple_conditions(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: Some(2),
            parent_id: Some(5),
            level: Some(TaskLevel::Trivial.to_int()),
            name: Some("PJ1 FULL OPT".to_string()),
            description: Some("PJ1 FULL OPT".to_string()),
            status: Some(TaskStatus::Done.to_int()),
            deadline_from: Some(99998),
            deadline_to: Some(100000),
            created_at_from: Some(99999),
            created_at_to: Some(99999),
            updated_at_from: Some(99999),
            updated_at_to: Some(99999),
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].project_id == 2);
        assert!(tasks[0].parent_id.unwrap() == 5);
        assert!(tasks[0].level == TaskLevel::Trivial.to_int());
        assert!(tasks[0].name == "Test PJ1 FULL OPT");
        assert!(tasks[0].description == Some("TEST PJ1 FULL OPT - Description".to_string()));
        assert!(tasks[0].status == TaskStatus::Done.to_int());
        assert!(tasks[0].deadline.unwrap() == 99999);
        assert!(tasks[0].created_at == 99999);
        assert!(tasks[0].updated_at.unwrap() == 99999);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_no_conditions(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 17);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_no_match(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: Some(1),
            parent_id: Some(2),
            level: Some(TaskLevel::Trivial.to_int()),
            name: Some("PJ1 Trivial".to_string()),
            description: Some("PJ1 Trivial TASK".to_string()),
            status: Some(TaskStatus::NotStarted.to_int()),
            deadline_from: Some(100000),
            deadline_to: Some(1),
            created_at_from: Some(99999),
            created_at_to: Some(99999),
            updated_at_from: Some(99999),
            updated_at_to: Some(99999),
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_update_task(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let updated_task = task_repo.get_task_by_id(1).await.unwrap();

        assert_eq!(updated_task.id, Some(1));
        assert_eq!(updated_task.project_id, 1);
        assert_eq!(updated_task.parent_id, None);
        assert_eq!(updated_task.level, TaskLevel::Major.to_int());
        assert_eq!(updated_task.name, "Test PJ0 Major TASK");
        assert_eq!(
            updated_task.description,
            Some("TEST PJ0 Major TASK - Description".to_string())
        );
        assert_eq!(updated_task.status, TaskStatus::NotStarted.to_int());
        assert_eq!(updated_task.deadline, Some(0));
        assert!(updated_task.updated_at.is_none());

        let task = Task {
            id: Some(1),
            project_id: 1,
            parent_id: None,
            level: TaskLevel::Major.to_int(),
            name: "Test Task".to_string(),
            description: Some("Test Task Description".to_string()),
            status: TaskStatus::Done.to_int(),
            deadline: Some(1000),
            created_at: 0,
            updated_at: None,
        };
        let updated_task = task_repo.update_task(task).await.unwrap();
        assert_eq!(updated_task.id, Some(1));
        assert_eq!(updated_task.project_id, 1);
        assert_eq!(updated_task.parent_id, None);
        assert_eq!(updated_task.level, TaskLevel::Major.to_int());
        assert_eq!(updated_task.name, "Test Task");
        assert_eq!(
            updated_task.description,
            Some("Test Task Description".to_string())
        );
        assert_eq!(updated_task.status, TaskStatus::Done.to_int());
        assert_eq!(updated_task.deadline, Some(1000));
        assert!(updated_task.updated_at.is_some());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_update_task_not_exists(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task {
            id: Some(100),
            project_id: 1,
            parent_id: None,
            level: TaskLevel::Major.to_int(),
            name: "Test Task".to_string(),
            description: Some("Test Task Description".to_string()),
            status: TaskStatus::NotStarted.to_int(),
            deadline: Some(1000),
            created_at: 0,
            updated_at: None,
        };
        let updated_task = task_repo.update_task(task).await;
        assert!(updated_task.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_delete_task(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);

        let found_task = task_repo.get_task_by_id(17).await.unwrap();
        assert_eq!(found_task.id, Some(17));

        let result = task_repo.delete_task(17).await.unwrap();
        assert_eq!(result, ());

        let not_found_task = task_repo.get_task_by_id(17).await;
        assert!(not_found_task.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_delete_task_not_exists(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let result = task_repo.delete_task(100).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_create_task_with_invalid_project_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task::new(
            999, // 存在しないプロジェクトID
            None,
            TaskLevel::Major.to_int(),
            "Test Task".to_string(),
            Some("Test Task Description".to_string()),
            TaskStatus::NotStarted.to_int(),
            None,
        );
        let result = task_repo.create_task(task).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_create_task_with_invalid_parent_level(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task::new(
            1,
            Some(1), // 親タスクのレベルが不正
            TaskLevel::Major.to_int(),
            "Test Task".to_string(),
            Some("Test Task Description".to_string()),
            TaskStatus::NotStarted.to_int(),
            None,
        );
        let result = task_repo.create_task(task).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_create_task_with_nonexistent_parent(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task::new(
            1,
            Some(999), // 存在しない親タスクID
            TaskLevel::Minor.to_int(),
            "Test Task".to_string(),
            Some("Test Task Description".to_string()),
            TaskStatus::NotStarted.to_int(),
            None,
        );
        let result = task_repo.create_task(task).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_update_task_with_invalid_project_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task {
            id: Some(1),
            project_id: 999, // 存在しないプロジェクトID
            parent_id: None,
            level: TaskLevel::Major.to_int(),
            name: "Test Task".to_string(),
            description: Some("Test Task Description".to_string()),
            status: TaskStatus::NotStarted.to_int(),
            deadline: None,
            created_at: 0,
            updated_at: None,
        };
        let result = task_repo.update_task(task).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_update_task_with_invalid_parent_level(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task {
            id: Some(1),
            project_id: 1,
            parent_id: Some(1), // 親タスクのレベルが不正
            level: TaskLevel::Major.to_int(),
            name: "Test Task".to_string(),
            description: Some("Test Task Description".to_string()),
            status: TaskStatus::NotStarted.to_int(),
            deadline: None,
            created_at: 0,
            updated_at: None,
        };
        let result = task_repo.update_task(task).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_update_task_with_nonexistent_parent(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task {
            id: Some(1),
            project_id: 1,
            parent_id: Some(999), // 存在しない親タスクID
            level: TaskLevel::Minor.to_int(),
            name: "Test Task".to_string(),
            description: Some("Test Task Description".to_string()),
            status: TaskStatus::NotStarted.to_int(),
            deadline: None,
            created_at: 0,
            updated_at: None,
        };
        let result = task_repo.update_task(task).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_update_task_with_self_as_parent(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let task = Task {
            id: Some(1),
            project_id: 1,
            parent_id: Some(1), // 自身のIDを親タスクIDとして指定
            level: TaskLevel::Minor.to_int(),
            name: "Test Task".to_string(),
            description: Some("Test Task Description".to_string()),
            status: TaskStatus::NotStarted.to_int(),
            deadline: None,
            created_at: 0,
            updated_at: None,
        };
        let result = task_repo.update_task(task).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_invalid_project_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: Some(-1), // 不正なプロジェクトID
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_invalid_parent_id(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: Some(-1), // 不正な親タスクID
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_invalid_level(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: Some(999), // 不正なタスクレベル
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_invalid_status(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: Some(999), // 不正なタスクステータス
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_empty_name(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: Some("".to_string()), // 空のタスク名
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_too_long_name(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: Some("a".repeat(129)), // 128文字超のタスク名
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_too_long_description(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: Some("a".repeat(1025)), // 1024文字超のタスク説明
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_tasks_by_filter_with_negative_timestamp(pool: SqlitePool) {
        let task_repo = TaskRepository::new(pool);
        let filter = TaskFilter {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: Some(-1), // 負のタイムスタンプ
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
        };
        let tasks = task_repo.get_tasks_by_filter(Some(&filter), None, None).await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_task_by_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let task = get_task_by_id_with_transaction(1, &mut tx)
            .await
            .unwrap();
        assert_eq!(task.id, Some(1));
    }

    #[sqlx::test(fixtures("tasks"))]
    async fn test_task_repo_get_task_by_id_with_transaction_not_found(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let result = get_task_by_id_with_transaction(100, &mut tx).await;
        assert!(result.is_err());
    }
}
