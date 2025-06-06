use crate::models::Project;
use crate::repository::project_repo::ProjectRepository;
use chrono::Utc;
use sqlx::sqlite::SqlitePool;

#[cfg(test)]
mod project_repo_test {
    use crate::repository::project_repo::get_project_by_id_with_transaction;

    use super::*;

    #[sqlx::test]
    async fn test_project_repo_create_project(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("create_test_{}", now);

        let project = Project {
            id: None,
            name: project_name.clone(),
        };

        let created_project = project_repo.create_project(project).await.unwrap();
        assert_eq!(created_project.name, project_name);
        assert!(created_project.id.is_some());

        let retrieved_project = project_repo
            .get_project_by_id(created_project.id.unwrap())
            .await
            .unwrap();
        assert_eq!(retrieved_project.name, project_name);
    }

    #[sqlx::test]
    async fn test_project_repo_get_project_by_id(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("get_by_id_test_{}", now);

        let project = Project {
            id: None,
            name: project_name.clone(),
        };

        let created_project = project_repo.create_project(project).await.unwrap();
        let retrieved_project = project_repo
            .get_project_by_id(created_project.id.unwrap())
            .await
            .unwrap();
        assert_eq!(retrieved_project.name, project_name);
    }

    #[sqlx::test]
    async fn test_project_repo_get_project_by_name(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("get_by_name_test_{}", now);

        let project = Project {
            id: None,
            name: project_name.clone(),
        };

        project_repo.create_project(project).await.unwrap();
        let retrieved_project = project_repo
            .get_project_by_name(&project_name)
            .await
            .unwrap();
        assert_eq!(retrieved_project.name, project_name);
    }

    #[sqlx::test]
    async fn test_project_repo_get_all_projects(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let project_count = 5;
        let projects = (1..=project_count)
            .map(|i| Project {
                id: None,
                name: format!("get_all_projects_test_{}_{}", now, i),
            })
            .collect::<Vec<Project>>();

        let mut created_projects = Vec::new();
        for project in projects.into_iter() {
            let created_project = project_repo.create_project(project).await.unwrap();
            created_projects.push(created_project);
        }

        let retrieved_projects = project_repo.get_all_projects().await.unwrap();
        assert_eq!(retrieved_projects.len(), project_count);

        for (i, project) in retrieved_projects.iter().enumerate() {
            assert_eq!(project.name, created_projects[i].name);
            assert_eq!(project.id, created_projects[i].id);
        }
    }

    #[sqlx::test]
    async fn test_project_repo_get_all_projects_empty(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let retrieved_projects = project_repo.get_all_projects().await.unwrap();
        assert!(retrieved_projects.is_empty());
    }

    #[sqlx::test]
    async fn test_project_repo_update_project(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("update_test_{}", now);

        let project = Project {
            id: None,
            name: project_name.clone(),
        };

        let created_project = project_repo.create_project(project).await.unwrap();

        let updated_project = Project {
            id: created_project.id,
            name: project_name.clone() + "_updated",
        };

        project_repo.update_project(updated_project).await.unwrap();

        let retrieved_project = project_repo
            .get_project_by_id(created_project.id.unwrap())
            .await
            .unwrap();
        assert_eq!(retrieved_project.name, project_name + "_updated");
        assert_eq!(retrieved_project.id, created_project.id);
    }

    #[sqlx::test]
    async fn test_project_repo_delete_project(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("delete_test_{}", now);

        let project = Project {
            id: None,
            name: project_name.clone(),
        };

        let created_project = project_repo.create_project(project).await.unwrap();
        let retrieved_project = project_repo
            .get_project_by_id(created_project.id.unwrap())
            .await;
        assert!(retrieved_project.is_ok());

        project_repo
            .delete_project(created_project.id.unwrap())
            .await
            .unwrap();
        let retrieved_project = project_repo
            .get_project_by_id(created_project.id.unwrap())
            .await;
        assert!(retrieved_project.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_create_duplicate_project(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("duplicate_test_1_{}", now);

        let project1 = Project {
            id: None,
            name: project_name.clone(),
        };

        let project2 = Project {
            id: None,
            name: project_name.clone(),
        };

        project_repo.create_project(project1).await.unwrap();
        let result = project_repo.create_project(project2).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_update_nonexistent_project(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("nonexistent_test_{}", now);

        let project = Project {
            id: Some(114514),
            name: project_name,
        };

        let result = project_repo.update_project(project).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_delete_nonexistent_project(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let result = project_repo.delete_project(114514).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_create_project_with_empty_name(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);

        let project = Project {
            id: None,
            name: "".to_string(),
        };

        let result = project_repo.create_project(project).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_create_project_with_long_name(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);

        let project = Project {
            id: None,
            name: "a".repeat(129),
        };

        let result = project_repo.create_project(project).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_update_project_with_empty_name(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("empty_name_test_{}", now);

        let project = Project {
            id: None,
            name: project_name.clone(),
        };

        let result = project_repo.create_project(project).await.unwrap();

        let updated_project = Project {
            id: result.id,
            name: "".to_string(),
        };

        let result = project_repo.update_project(updated_project).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_update_project_with_long_name(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("long_name_test_{}", now);

        let project = Project {
            id: None,
            name: project_name.clone(),
        };

        let result = project_repo.create_project(project).await.unwrap();

        let updated_project = Project {
            id: result.id,
            name: "a".repeat(129),
        };

        let result = project_repo.update_project(updated_project).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_create_project_with_invalid_id(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("invalid_id_test_{}", now);

        let project = Project {
            id: Some(-1),
            name: project_name,
        };

        let result = project_repo.create_project(project).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_project_repo_update_project_with_invalid_id(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = format!("invalid_id_test_{}", now);

        let project = Project {
            id: Some(0),
            name: project_name,
        };

        let result = project_repo.update_project(project).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("projects"))]
    async fn test_project_repo_get_project_by_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let project = get_project_by_id_with_transaction(1, &mut tx)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(project.name, "Test Project 0");
    }

    #[sqlx::test(fixtures("projects"))]
    async fn test_project_repo_get_project_by_id_with_transaction_not_found(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let project = get_project_by_id_with_transaction(100, &mut tx)
            .await
            .unwrap();
        assert!(project.is_none());
    }

    #[sqlx::test(fixtures("projects"))]
    async fn test_project_repo_get_projects_count(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let count = project_repo.get_projects_count().await.unwrap();
        assert_eq!(count, 10);
    }

    #[sqlx::test(fixtures("projects"))]
    async fn test_project_repo_get_projects_with_pagenation(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let retrieved_projects = project_repo.get_projects_with_pagenation(&1, &5).await.unwrap();
        assert_eq!(retrieved_projects.len(), 5);
        assert_eq!(retrieved_projects[0].name, "Test Project 0");
        assert_eq!(retrieved_projects[1].name, "Test Project 1");
        assert_eq!(retrieved_projects[2].name, "Test Project 2");
        assert_eq!(retrieved_projects[3].name, "Test Project 3");
        assert_eq!(retrieved_projects[4].name, "Test Project 4");

        let retrieved_projects = project_repo.get_projects_with_pagenation(&2, &5).await.unwrap();
        assert_eq!(retrieved_projects.len(), 5);
        assert_eq!(retrieved_projects[0].name, "Test Project 5");
        assert_eq!(retrieved_projects[1].name, "Test Project 6");
        assert_eq!(retrieved_projects[2].name, "Test Project 7");
        assert_eq!(retrieved_projects[3].name, "Test Project 8");
        assert_eq!(retrieved_projects[4].name, "Test Project 9");
    }

    #[sqlx::test(fixtures("projects"))]
    async fn test_project_repo_get_projects_with_pagenation_invalid_page(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let retrieved_projects = project_repo.get_projects_with_pagenation(&100, &5).await;
        assert!(retrieved_projects.is_err());
    }

    #[sqlx::test(fixtures("projects"))]
    async fn test_project_repo_get_user_by_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let project = get_project_by_id_with_transaction(1, &mut tx)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(project.name, "Test Project 0");
    }

    #[sqlx::test(fixtures("projects"))]
    async fn test_project_repo_get_user_count(pool: SqlitePool) {
        let project_repo = ProjectRepository::new(pool);
        let count = project_repo.get_projects_count().await.unwrap();
        assert_eq!(count, 10);
    }
}
