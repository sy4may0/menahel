use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::models::TaskWithUser;
use crate::models::Project;
use crate::models::ProjectResponse;
use crate::models::ErrorResponse;
use crate::client::repository::TaskTree;

use reqwest::StatusCode;
use anyhow::Result;

static API_URL: Lazy<String> = Lazy::new(|| {
    let url = std::env::var("MENAHEL_API_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    url
});

pub async fn get_project(project_name: &str) -> Result<Project> {
    let url = format!("{}/projects?target=name&name={}", API_URL.as_str(), project_name);
    let response = reqwest::get(url).await;

    let response = match response {
        Ok(response) => response,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to get project: {:?}", e.to_string()));
        }
    };

    let project = match response.status() {
        StatusCode::OK => {
            let project_response: ProjectResponse = response.json().await?;
            if project_response.results.is_empty() {
                return Err(anyhow::anyhow!("Project not found"));
            }
            project_response.results[0].clone()
        }
        _ => {
            let error_response: Result<ErrorResponse, reqwest::Error> = response.json().await;
            match error_response {
                Ok(error_response) => {
                    return Err(anyhow::anyhow!(error_response.message));
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }
        }
    };

    Ok(project)
}

pub async fn get_task_tree(project_id: i64) -> Result<HashMap<i64, TaskTree>> {
    let level0_tasks_url = format!("{}/tasks?target=filter&project_id={}&level=0&with_user=true", API_URL.as_str(), project_id);
    let level1_tasks_url = format!("{}/tasks?target=filter&project_id={}&level=1&with_user=true", API_URL.as_str(), project_id);

    // レベル0タスクの取得
    let level0_tasks_response = reqwest::get(&level0_tasks_url).await?;
    let level0_tasks = match level0_tasks_response.status() {
        StatusCode::OK => {
            level0_tasks_response.json::<Vec<TaskWithUser>>().await?
        }
        _ => {
            let error_response: Result<ErrorResponse, reqwest::Error> = level0_tasks_response.json().await;
            match error_response {
                Ok(error_response) => {
                    return Err(anyhow::anyhow!("Failed to get level 0 tasks: {}", error_response.message));
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to get level 0 tasks: {}", e.to_string()));
                }
            }
        }
    };

    // レベル1タスクの取得
    let level1_tasks_response = reqwest::get(&level1_tasks_url).await?;
    let level1_tasks = match level1_tasks_response.status() {
        StatusCode::OK => {
            level1_tasks_response.json::<Vec<TaskWithUser>>().await?
        }
        _ => {
            let error_response: Result<ErrorResponse, reqwest::Error> = level1_tasks_response.json().await;
            match error_response {
                Ok(error_response) => {
                    return Err(anyhow::anyhow!("Failed to get level 1 tasks: {}", error_response.message));
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to get level 1 tasks: {}", e.to_string()));
                }
            }
        }
    };

    let mut task_map: HashMap<i64, TaskTree> = HashMap::new();

    // レベル0タスクを初期化
    for task in level0_tasks {
        let task_tree = TaskTree {
            own: task.clone(),
            children: Some(HashMap::new()),
        };
        task_map.insert(task.task_id, task_tree);
    }

    // レベル1タスクを親タスクに追加
    for task in level1_tasks {
        let parent_id = match task.parent_id {
            Some(id) => id,
            None => {
                // 親IDがない場合はスキップ（ログ出力を追加することも可能）
                continue;
            }
        };
        
        if let Some(parent_task) = task_map.get_mut(&parent_id) {
            if let Some(children) = parent_task.children.as_mut() {
                children.insert(task.task_id, TaskTree {
                    own: task,
                    children: Some(HashMap::new()), // 将来の拡張性のため空のHashMapを設定
                });
            }
        } else {
            // 親タスクが見つからない場合（データの不整合の可能性）
            return Err(anyhow::anyhow!("Parent task with ID {} not found for task {}", parent_id, task.task_id));
        }
    }

    Ok(task_map)
}