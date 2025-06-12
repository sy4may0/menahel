use crate::errors::messages::ErrorKey;
use std::collections::HashMap;

pub fn add_task_user_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    let mut task_user_get_all_failed = HashMap::new();
    task_user_get_all_failed.insert("en", "Failed to get all tasks and users due to database operation failure");
    task_user_get_all_failed.insert("jp", "DB操作処理の問題により全てのタスクとユーザーの取得に失敗しました");
    map.insert(ErrorKey::TaskUserGetAllFailed, task_user_get_all_failed);

    let mut task_user_get_by_filter_failed = HashMap::new();
    task_user_get_by_filter_failed.insert("en", "Failed to get tasks and users by filter due to database operation failure");
    task_user_get_by_filter_failed.insert("jp", "DB操作処理の問題によりフィルターに基づいてタスクとユーザーを取得に失敗しました");
    map.insert(ErrorKey::TaskUserGetByFilterFailed, task_user_get_by_filter_failed);

    let mut task_user_get_pagination_not_found = HashMap::new();
    task_user_get_pagination_not_found.insert("en", "No tasks and users found on the specified page");
    task_user_get_pagination_not_found.insert("jp", "指定ページにタスクとユーザーが存在しません");
    map.insert(ErrorKey::TaskUserGetPaginationNotFound, task_user_get_pagination_not_found);
}