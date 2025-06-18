use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_user_assign_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    // ユーザー割り当て関連のエラーメッセージ
    let mut user_assign_create_failed = HashMap::new();
    user_assign_create_failed.insert(
        "en",
        "Failed to create user assign due to database operation failure",
    );
    user_assign_create_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザー割り当ての作成に失敗しました",
    );
    map.insert(ErrorKey::UserAssignCreateFailed, user_assign_create_failed);

    let mut user_assign_id_must_be_none = HashMap::new();
    user_assign_id_must_be_none.insert("en", "User assign ID must be none");
    user_assign_id_must_be_none.insert("jp", "ユーザー割り当てIDはnullでなければなりません");
    map.insert(
        ErrorKey::UserAssignIdMustBeNone,
        user_assign_id_must_be_none,
    );

    let mut user_assign_get_by_id_failed = HashMap::new();
    user_assign_get_by_id_failed.insert(
        "en",
        "Failed to get user assign by ID due to database operation failure",
    );
    user_assign_get_by_id_failed.insert(
        "jp",
        "DB操作処理の問題によりIDによるユーザー割り当ての取得に失敗しました",
    );
    map.insert(
        ErrorKey::UserAssignGetByIdFailed,
        user_assign_get_by_id_failed,
    );

    let mut user_assign_get_by_task_id_failed = HashMap::new();
    user_assign_get_by_task_id_failed.insert(
        "en",
        "Failed to get user assign by task ID due to database operation failure",
    );
    user_assign_get_by_task_id_failed.insert(
        "jp",
        "DB操作処理の問題によりタスクIDによるユーザー割り当ての取得に失敗しました",
    );
    map.insert(
        ErrorKey::UserAssignGetByTaskIdFailed,
        user_assign_get_by_task_id_failed,
    );

    let mut user_assign_get_by_user_id_failed = HashMap::new();
    user_assign_get_by_user_id_failed.insert(
        "en",
        "Failed to get user assign by user ID due to database operation failure",
    );
    user_assign_get_by_user_id_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザーIDによるユーザー割り当ての取得に失敗しました",
    );
    map.insert(
        ErrorKey::UserAssignGetByUserIdFailed,
        user_assign_get_by_user_id_failed,
    );

    let mut user_assign_get_by_user_id_and_task_id_failed = HashMap::new();
    user_assign_get_by_user_id_and_task_id_failed.insert(
        "en",
        "Failed to get user assign by user ID and task ID due to database operation failure",
    );
    user_assign_get_by_user_id_and_task_id_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザーIDとタスクIDによるユーザー割り当ての取得に失敗しました",
    );
    map.insert(
        ErrorKey::UserAssignGetByUserIdAndTaskIdFailed,
        user_assign_get_by_user_id_and_task_id_failed,
    );

    let mut user_assign_get_all_failed = HashMap::new();
    user_assign_get_all_failed.insert(
        "en",
        "Failed to get all user assigns due to database operation failure",
    );
    user_assign_get_all_failed.insert(
        "jp",
        "DB操作処理の問題によりすべてのユーザー割り当ての取得に失敗しました",
    );
    map.insert(ErrorKey::UserAssignGetAllFailed, user_assign_get_all_failed);

    let mut user_assign_update_failed = HashMap::new();
    user_assign_update_failed.insert(
        "en",
        "Failed to update user assign due to database operation failure",
    );
    user_assign_update_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザー割り当ての更新に失敗しました",
    );
    map.insert(ErrorKey::UserAssignUpdateFailed, user_assign_update_failed);

    let mut user_assign_delete_failed = HashMap::new();
    user_assign_delete_failed.insert(
        "en",
        "Failed to delete user assign due to database operation failure",
    );
    user_assign_delete_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザー割り当ての削除に失敗しました",
    );
    map.insert(ErrorKey::UserAssignDeleteFailed, user_assign_delete_failed);

    let mut user_assign_delete_failed_by_id_not_found = HashMap::new();
    user_assign_delete_failed_by_id_not_found.insert(
        "en",
        "Failed to delete user assign becouse user assign does not exist",
    );
    user_assign_delete_failed_by_id_not_found
        .insert("jp", "存在しないユーザー割り当てを削除しようとしました。");
    map.insert(
        ErrorKey::UserAssignDeleteFailedByIdNotFound,
        user_assign_delete_failed_by_id_not_found,
    );

    let mut user_assign_user_id_invalid = HashMap::new();
    user_assign_user_id_invalid.insert("en", "User ID must be greater than 0");
    user_assign_user_id_invalid.insert("jp", "ユーザーIDは0より大きくなければなりません");
    map.insert(
        ErrorKey::UserAssignUserIdInvalid,
        user_assign_user_id_invalid,
    );

    let mut user_assign_task_id_invalid = HashMap::new();
    user_assign_task_id_invalid.insert("en", "Task ID must be greater than 0");
    user_assign_task_id_invalid.insert("jp", "タスクIDは0より大きくなければなりません");
    map.insert(
        ErrorKey::UserAssignTaskIdInvalid,
        user_assign_task_id_invalid,
    );

    let mut user_assign_user_id_not_found = HashMap::new();
    user_assign_user_id_not_found.insert("en", "The user assign to task does not exist");
    user_assign_user_id_not_found.insert(
        "jp",
        "存在しないユーザーをタスクに割り当てしようとしました。",
    );
    map.insert(
        ErrorKey::UserAssignUserIdNotFound,
        user_assign_user_id_not_found,
    );

    let mut user_assign_task_id_not_found = HashMap::new();
    user_assign_task_id_not_found.insert("en", "The task assign to user does not exist");
    user_assign_task_id_not_found.insert(
        "jp",
        "存在しないタスクをユーザーに割り当てしようとしました。",
    );
    map.insert(
        ErrorKey::UserAssignTaskIdNotFound,
        user_assign_task_id_not_found,
    );

    let mut user_assign_to_not_max_level_task = HashMap::new();
    user_assign_to_not_max_level_task.insert(
        "en",
        "The user can only be assigned to tasks at the maximum level",
    );
    user_assign_to_not_max_level_task.insert(
        "jp",
        "ユーザーは最も詳細な階層のタスクにしか割り当てられません",
    );
    map.insert(
        ErrorKey::UserAssignToNotMaxLevelTask,
        user_assign_to_not_max_level_task,
    );

    let mut user_assign_same_user_assign_exists = HashMap::new();
    user_assign_same_user_assign_exists.insert("en", "The user is already assigned to the task");
    user_assign_same_user_assign_exists
        .insert("jp", "すでにユーザーはタスクに割り当てられています");
    map.insert(
        ErrorKey::UserAssignSameUserAssignExists,
        user_assign_same_user_assign_exists,
    );

    let mut user_assign_get_by_id_not_found = HashMap::new();
    user_assign_get_by_id_not_found.insert("en", "User assign not found");
    user_assign_get_by_id_not_found.insert("jp", "ユーザー割り当てが見つかりません");
    map.insert(
        ErrorKey::UserAssignGetByIdNotFound,
        user_assign_get_by_id_not_found,
    );

    let mut user_assign_get_pagination_not_found = HashMap::new();
    user_assign_get_pagination_not_found
        .insert("en", "No user assigns found in the specified page");
    user_assign_get_pagination_not_found
        .insert("jp", "指定ページ内にユーザー割り当てが存在しません。");
    map.insert(
        ErrorKey::UserAssignGetPaginationNotFound,
        user_assign_get_pagination_not_found,
    );

    let mut user_assign_get_by_user_id_and_task_id_not_found = HashMap::new();
    user_assign_get_by_user_id_and_task_id_not_found.insert("en", "User assign not found");
    user_assign_get_by_user_id_and_task_id_not_found
        .insert("jp", "ユーザー割り当てが見つかりません");
    map.insert(
        ErrorKey::UserAssignGetByUserIdAndTaskIdNotFound,
        user_assign_get_by_user_id_and_task_id_not_found,
    );

    let mut user_assign_get_user_assigns_count_failed = HashMap::new();
    user_assign_get_user_assigns_count_failed.insert(
        "en",
        "Failed to get user assigns count due to database operation failure",
    );
    user_assign_get_user_assigns_count_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザー割り当ての数の取得に失敗しました",
    );
    map.insert(
        ErrorKey::UserAssignGetUserAssignsCountFailed,
        user_assign_get_user_assigns_count_failed,
    );

    let mut user_assign_id_invalid = HashMap::new();
    user_assign_id_invalid.insert("en", "User assign ID must be greater than 0");
    user_assign_id_invalid.insert("jp", "ユーザー割り当てIDは0より大きくなければなりません");
    map.insert(ErrorKey::UserAssignIdInvalid, user_assign_id_invalid);
}
