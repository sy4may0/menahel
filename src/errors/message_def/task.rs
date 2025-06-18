use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_task_error_messages(map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>) {
    // タスク関連のエラーメッセージ
    let mut task_id_invalid = HashMap::new();
    task_id_invalid.insert("en", "Task ID must be greater than 0");
    task_id_invalid.insert("jp", "タスクIDは0より大きくなければなりません");
    map.insert(ErrorKey::TaskIdInvalid, task_id_invalid);

    let mut task_id_must_be_none = HashMap::new();
    task_id_must_be_none.insert("en", "Task ID must be none");
    task_id_must_be_none.insert("jp", "タスクIDはnullでなければなりません");
    map.insert(ErrorKey::TaskIdMustBeNone, task_id_must_be_none);

    let mut task_project_id_invalid = HashMap::new();
    task_project_id_invalid.insert("en", "Project ID must be greater than 0");
    task_project_id_invalid.insert("jp", "プロジェクトIDは0より大きくなければなりません");
    map.insert(ErrorKey::TaskProjectIdInvalid, task_project_id_invalid);

    let mut task_parent_id_invalid = HashMap::new();
    task_parent_id_invalid.insert("en", "Parent ID must be greater than 0");
    task_parent_id_invalid.insert("jp", "親タスクIDは0より大きくなければなりません");
    map.insert(ErrorKey::TaskParentIdInvalid, task_parent_id_invalid);

    let mut task_level_invalid = HashMap::new();
    task_level_invalid.insert("en", "Invalid task level");
    task_level_invalid.insert("jp", "無効なタスクレベルです");
    map.insert(ErrorKey::TaskLevelInvalid, task_level_invalid);

    let mut task_status_invalid = HashMap::new();
    task_status_invalid.insert("en", "Invalid task status");
    task_status_invalid.insert("jp", "無効なタスクステータスです");
    map.insert(ErrorKey::TaskStatusInvalid, task_status_invalid);

    let mut task_name_empty = HashMap::new();
    task_name_empty.insert("en", "Task name cannot be empty");
    task_name_empty.insert("jp", "タスク名は空にできません");
    map.insert(ErrorKey::TaskNameEmpty, task_name_empty);

    let mut task_name_too_long = HashMap::new();
    task_name_too_long.insert("en", "Task name cannot be longer than 128 characters");
    task_name_too_long.insert("jp", "タスク名は128文字以下である必要があります");
    map.insert(ErrorKey::TaskNameTooLong, task_name_too_long);

    let mut task_description_too_long = HashMap::new();
    task_description_too_long.insert(
        "en",
        "Task description cannot be longer than 1024 characters",
    );
    task_description_too_long.insert("jp", "タスクの説明は1024文字以下である必要があります");
    map.insert(ErrorKey::TaskDescriptionTooLong, task_description_too_long);

    let mut task_timestamp_invalid = HashMap::new();
    task_timestamp_invalid.insert("en", "Timestamp must be greater than 0");
    task_timestamp_invalid.insert("jp", "タイムスタンプは0より大きくなければなりません");
    map.insert(ErrorKey::TaskTimestampInvalid, task_timestamp_invalid);

    let mut task_timestamp_or_none_invalid = HashMap::new();
    task_timestamp_or_none_invalid.insert("en", "Timestamp must be greater than 0 or None");
    task_timestamp_or_none_invalid.insert(
        "jp",
        "タイムスタンプは0より大きいか、Noneである必要があります",
    );
    map.insert(
        ErrorKey::TaskTimestampOrNoneInvalid,
        task_timestamp_or_none_invalid,
    );

    let mut task_project_id_not_found = HashMap::new();
    task_project_id_not_found.insert("en", "Project not found.");
    task_project_id_not_found.insert("jp", "プロジェクトが見つかりません");
    map.insert(ErrorKey::TaskProjectIdNotFound, task_project_id_not_found);

    let mut task_no_parent_id_on_non_major_task = HashMap::new();
    task_no_parent_id_on_non_major_task.insert("en", "Parent ID is required for non-major tasks");
    task_no_parent_id_on_non_major_task
        .insert("jp", "大項目タスク以外のタスクには親タスクIDが必要です");
    map.insert(
        ErrorKey::TaskNoParentIdOnNonMajorTask,
        task_no_parent_id_on_non_major_task,
    );

    let mut task_parent_id_not_found = HashMap::new();
    task_parent_id_not_found.insert("en", "Parent task not found");
    task_parent_id_not_found.insert("jp", "親タスクが見つかりません");
    map.insert(ErrorKey::TaskParentIdNotFound, task_parent_id_not_found);

    let mut task_parent_level_invalid = HashMap::new();
    task_parent_level_invalid.insert("en", "Parent task level is not one level higher");
    task_parent_level_invalid.insert("jp", "親タスクのレベルが1つ上ではありません");
    map.insert(ErrorKey::TaskParentLevelInvalid, task_parent_level_invalid);

    let mut task_parent_id_cannot_be_same_as_task_id = HashMap::new();
    task_parent_id_cannot_be_same_as_task_id
        .insert("en", "Parent ID cannot be the same as the task ID");
    task_parent_id_cannot_be_same_as_task_id.insert("jp", "親タスクIDはタスクIDと同じにできません");
    map.insert(
        ErrorKey::TaskParentIdCannotBeSameAsTaskId,
        task_parent_id_cannot_be_same_as_task_id,
    );

    let mut task_create_failed = HashMap::new();
    task_create_failed.insert(
        "en",
        "Failed to create task due to database operation failure",
    );
    task_create_failed.insert("jp", "DB操作処理の問題によりタスクの作成に失敗しました");
    map.insert(ErrorKey::TaskCreateFailed, task_create_failed);

    let mut task_get_by_id_failed = HashMap::new();
    task_get_by_id_failed.insert(
        "en",
        "Failed to get task by ID due to database operation failure",
    );
    task_get_by_id_failed.insert(
        "jp",
        "DB操作処理の問題によりIDによるタスクの取得に失敗しました",
    );
    map.insert(ErrorKey::TaskGetByIdFailed, task_get_by_id_failed);

    let mut task_get_all_failed = HashMap::new();
    task_get_all_failed.insert(
        "en",
        "Failed to get all tasks due to database operation failure",
    );
    task_get_all_failed.insert(
        "jp",
        "DB操作処理の問題によりすべてのタスクの取得に失敗しました",
    );
    map.insert(ErrorKey::TaskGetAllFailed, task_get_all_failed);

    let mut task_get_by_filter_failed = HashMap::new();
    task_get_by_filter_failed.insert(
        "en",
        "Failed to get tasks by filter due to database operation failure",
    );
    task_get_by_filter_failed.insert(
        "jp",
        "DB操作処理の問題によりフィルターによるタスクの取得に失敗しました",
    );
    map.insert(ErrorKey::TaskGetByFilterFailed, task_get_by_filter_failed);

    let mut task_update_failed = HashMap::new();
    task_update_failed.insert(
        "en",
        "Failed to update task due to database operation failure",
    );
    task_update_failed.insert("jp", "DB操作処理の問題によりタスクの更新に失敗しました");
    map.insert(ErrorKey::TaskUpdateFailed, task_update_failed);

    let mut task_delete_failed = HashMap::new();
    task_delete_failed.insert(
        "en",
        "Failed to delete task due to database operation failure",
    );
    task_delete_failed.insert("jp", "DB操作処理の問題によりタスクの削除に失敗しました");
    map.insert(ErrorKey::TaskDeleteFailed, task_delete_failed);

    let mut task_delete_failed_by_id_not_found = HashMap::new();
    task_delete_failed_by_id_not_found
        .insert("en", "Failed to delete task becouse task does not exist");
    task_delete_failed_by_id_not_found.insert("jp", "存在しないタスクを削除しようとしました。");
    map.insert(
        ErrorKey::TaskDeleteFailedByIdNotFound,
        task_delete_failed_by_id_not_found,
    );

    let mut task_get_count_failed = HashMap::new();
    task_get_count_failed.insert(
        "en",
        "Failed to get count of tasks due to database operation failure",
    );
    task_get_count_failed.insert(
        "jp",
        "DB操作処理の問題によりタスクの件数の取得に失敗しました",
    );
    map.insert(ErrorKey::TaskGetCountFailed, task_get_count_failed);

    let mut task_get_pagination_not_found = HashMap::new();
    task_get_pagination_not_found.insert("en", "No tasks found in the specified page");
    task_get_pagination_not_found.insert("jp", "指定ページ内にタスクが存在しません。");
    map.insert(
        ErrorKey::TaskGetPaginationNotFound,
        task_get_pagination_not_found,
    );

    let mut task_get_by_id_not_found = HashMap::new();
    task_get_by_id_not_found.insert("en", "Task not found");
    task_get_by_id_not_found.insert("jp", "タスクが見つかりません");
    map.insert(ErrorKey::TaskGetByIdNotFound, task_get_by_id_not_found);
}
