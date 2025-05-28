use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKey {
    // ユーザー関連のエラー
    UserIdInvalid,
    UserNameEmpty,
    UserNameTooLong,
    UserNameContainsInvalidCharacters,
    UserEmailEmpty,
    UserEmailTooLong,
    UserEmailInvalid,
    UserPasswordEmpty,
    UserPasswordInvalid,
    UserCreateFailed,
    UserGetByIdFailed,
    UserGetByNameFailed,
    UserGetAllFailed,
    UserUpdateFailed,
    UserDeleteFailed,
    UserDeleteFailedByIdNotFound,
    
    // プロジェクト関連のエラー
    ProjectIdInvalid,
    ProjectNameEmpty,
    ProjectNameTooLong,
    ProjectCreateFailed,
    ProjectGetByIdFailed,
    ProjectGetByNameFailed,
    ProjectGetAllFailed,
    ProjectUpdateFailed,
    ProjectDeleteFailed,
    ProjectDeleteFailedByIdNotFound,

    // タスク関連のエラー
    TaskIdInvalid,
    TaskProjectIdInvalid,
    TaskParentIdInvalid,
    TaskLevelInvalid,
    TaskStatusInvalid,
    TaskNameEmpty,
    TaskNameTooLong,
    TaskDescriptionTooLong,
    TaskTimestampInvalid,
    TaskTimestampOrNoneInvalid,
    TaskProjectIdNotFound,
    TaskNoParentIdOnNonMajorTask,
    TaskParentIdNotFound,
    TaskParentLevelInvalid,
    TaskParentIdCannotBeSameAsTaskId,
    TaskCreateFailed,
    TaskGetByIdFailed,
    TaskGetAllFailed,
    TaskGetByFilterFailed,
    TaskUpdateFailed,
    TaskDeleteFailed,
    TaskDeleteFailedByIdNotFound,

    // ユーザー割り当て関連のエラー
    UserAssignCreateFailed,
    UserAssignGetByIdFailed,
    UserAssignGetByTaskIdFailed,
    UserAssignGetByUserIdFailed,
    UserAssignGetByUserIdAndTaskIdFailed,
    UserAssignGetAllFailed,
    UserAssignUpdateFailed,
    UserAssignDeleteFailed,
    UserAssignDeleteFailedByIdNotFound,
    UserAssignUserIdInvalid,
    UserAssignTaskIdInvalid,
    UserAssignUserIdNotFound,
    UserAssignTaskIdNotFound,
    UserAssignToNotMaxLevelTask,
    UserAssignSameUserAssignExists,
}

impl fmt::Display for ErrorKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ユーザー関連のエラー
            ErrorKey::UserIdInvalid => write!(f, "UserIdInvalid"),
            ErrorKey::UserNameEmpty => write!(f, "UserNameEmpty"),
            ErrorKey::UserNameTooLong => write!(f, "UserNameTooLong"),
            ErrorKey::UserNameContainsInvalidCharacters => write!(f, "UserNameContainsInvalidCharacters"),
            ErrorKey::UserEmailEmpty => write!(f, "UserEmailEmpty"),
            ErrorKey::UserEmailTooLong => write!(f, "UserEmailTooLong"),
            ErrorKey::UserEmailInvalid => write!(f, "UserEmailInvalid"),
            ErrorKey::UserPasswordEmpty => write!(f, "UserPasswordEmpty"),
            ErrorKey::UserPasswordInvalid => write!(f, "UserPasswordInvalid"),
            ErrorKey::UserCreateFailed => write!(f, "UserCreateFailed"),
            ErrorKey::UserGetByIdFailed => write!(f, "UserGetByIdFailed"),
            ErrorKey::UserGetByNameFailed => write!(f, "UserGetByNameFailed"),
            ErrorKey::UserGetAllFailed => write!(f, "UserGetAllFailed"),
            ErrorKey::UserUpdateFailed => write!(f, "UserUpdateFailed"),
            ErrorKey::UserDeleteFailed => write!(f, "UserDeleteFailed"),
            ErrorKey::UserDeleteFailedByIdNotFound => write!(f, "UserDeleteFailedByIdNotFound"),
            
            // プロジェクト関連のエラー
            ErrorKey::ProjectIdInvalid => write!(f, "ProjectIdInvalid"),
            ErrorKey::ProjectNameEmpty => write!(f, "ProjectNameEmpty"),
            ErrorKey::ProjectNameTooLong => write!(f, "ProjectNameTooLong"),
            ErrorKey::ProjectCreateFailed => write!(f, "ProjectCreateFailed"),
            ErrorKey::ProjectGetByIdFailed => write!(f, "ProjectGetByIdFailed"),
            ErrorKey::ProjectGetByNameFailed => write!(f, "ProjectGetByNameFailed"),
            ErrorKey::ProjectGetAllFailed => write!(f, "ProjectGetAllFailed"),
            ErrorKey::ProjectUpdateFailed => write!(f, "ProjectUpdateFailed"),
            ErrorKey::ProjectDeleteFailed => write!(f, "ProjectDeleteFailed"),
            ErrorKey::ProjectDeleteFailedByIdNotFound => write!(f, "ProjectDeleteFailedByIdNotFound"),

            // タスク関連のエラー
            ErrorKey::TaskIdInvalid => write!(f, "TaskIdInvalid"),
            ErrorKey::TaskProjectIdInvalid => write!(f, "TaskProjectIdInvalid"),
            ErrorKey::TaskParentIdInvalid => write!(f, "TaskParentIdInvalid"),
            ErrorKey::TaskLevelInvalid => write!(f, "TaskLevelInvalid"),
            ErrorKey::TaskStatusInvalid => write!(f, "TaskStatusInvalid"),
            ErrorKey::TaskNameEmpty => write!(f, "TaskNameEmpty"),
            ErrorKey::TaskNameTooLong => write!(f, "TaskNameTooLong"),
            ErrorKey::TaskDescriptionTooLong => write!(f, "TaskDescriptionTooLong"),
            ErrorKey::TaskTimestampInvalid => write!(f, "TaskTimestampInvalid"),
            ErrorKey::TaskTimestampOrNoneInvalid => write!(f, "TaskTimestampOrNoneInvalid"),
            ErrorKey::TaskProjectIdNotFound => write!(f, "TaskProjectIdNotFound"),
            ErrorKey::TaskNoParentIdOnNonMajorTask => write!(f, "TaskNoParentIdOnNonMajorTask"),
            ErrorKey::TaskParentIdNotFound => write!(f, "TaskParentIdNotFound"),
            ErrorKey::TaskParentLevelInvalid => write!(f, "TaskParentLevelInvalid"),
            ErrorKey::TaskParentIdCannotBeSameAsTaskId => write!(f, "TaskParentIdCannotBeSameAsTaskId"),
            ErrorKey::TaskCreateFailed => write!(f, "TaskCreateFailed"),
            ErrorKey::TaskGetByIdFailed => write!(f, "TaskGetByIdFailed"),
            ErrorKey::TaskGetAllFailed => write!(f, "TaskGetAllFailed"),
            ErrorKey::TaskGetByFilterFailed => write!(f, "TaskGetByFilterFailed"),
            ErrorKey::TaskUpdateFailed => write!(f, "TaskUpdateFailed"),
            ErrorKey::TaskDeleteFailed => write!(f, "TaskDeleteFailed"),
            ErrorKey::TaskDeleteFailedByIdNotFound => write!(f, "TaskDeleteFailedByIdNotFound"),

            // ユーザー割り当て関連のエラー
            ErrorKey::UserAssignCreateFailed => write!(f, "UserAssignCreateFailed"),
            ErrorKey::UserAssignGetByIdFailed => write!(f, "UserAssignGetByIdFailed"),
            ErrorKey::UserAssignGetByTaskIdFailed => write!(f, "UserAssignGetByTaskIdFailed"),
            ErrorKey::UserAssignGetAllFailed => write!(f, "UserAssignGetAllFailed"),
            ErrorKey::UserAssignUpdateFailed => write!(f, "UserAssignUpdateFailed"),
            ErrorKey::UserAssignDeleteFailed => write!(f, "UserAssignDeleteFailed"),
            ErrorKey::UserAssignDeleteFailedByIdNotFound => write!(f, "UserAssignDeleteFailedByIdNotFound"),
            ErrorKey::UserAssignUserIdInvalid => write!(f, "UserAssignUserIdInvalid"),
            ErrorKey::UserAssignTaskIdInvalid => write!(f, "UserAssignTaskIdInvalid"),
            ErrorKey::UserAssignUserIdNotFound => write!(f, "UserAssignUserIdNotFound"),
            ErrorKey::UserAssignTaskIdNotFound => write!(f, "UserAssignTaskIdNotFound"),
            ErrorKey::UserAssignToNotMaxLevelTask => write!(f, "UserAssignToNotMaxLevelTask"),
            ErrorKey::UserAssignSameUserAssignExists => write!(f, "UserAssignSameUserAssignExists"),
            ErrorKey::UserAssignGetByUserIdFailed => write!(f, "UserAssignGetByUserIdFailed"),
            ErrorKey::UserAssignGetByUserIdAndTaskIdFailed => write!(f, "UserAssignGetByUserIdAndTaskIdFailed"),
        }
    }
}

static GLOBAL_LANG: Lazy<&'static str> = Lazy::new(|| {
    //[TODO] Configから取得するようにする
    "en"
});

static ERROR_MESSAGES: Lazy<HashMap<ErrorKey, HashMap<&'static str, &'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // ユーザー関連のエラーメッセージ
    let mut user_id_invalid = HashMap::new();
    user_id_invalid.insert("en", "User ID must be greater than 0");
    user_id_invalid.insert("jp", "ユーザーIDは0より大きくなければなりません");
    map.insert(ErrorKey::UserIdInvalid, user_id_invalid);

    let mut user_name_empty = HashMap::new();
    user_name_empty.insert("en", "Username cannot be empty");
    user_name_empty.insert("jp", "ユーザー名は空にできません");
    map.insert(ErrorKey::UserNameEmpty, user_name_empty);

    let mut user_name_too_long = HashMap::new();
    user_name_too_long.insert("en", "Username cannot be longer than 128 characters");
    user_name_too_long.insert("jp", "ユーザー名は128文字以下である必要があります");
    map.insert(ErrorKey::UserNameTooLong, user_name_too_long);

    let mut user_name_invalid_chars = HashMap::new();
    user_name_invalid_chars.insert("en", "Username can only contain alphanumeric characters, dots, and underscores");
    user_name_invalid_chars.insert("jp", "ユーザー名には英数字、ドット、アンダースコアのみ使用できます");
    map.insert(ErrorKey::UserNameContainsInvalidCharacters, user_name_invalid_chars);

    let mut user_email_empty = HashMap::new();
    user_email_empty.insert("en", "Email cannot be empty");
    user_email_empty.insert("jp", "メールアドレスは空にできません");
    map.insert(ErrorKey::UserEmailEmpty, user_email_empty);

    let mut user_email_too_long = HashMap::new();
    user_email_too_long.insert("en", "Email cannot be longer than 254 characters");
    user_email_too_long.insert("jp", "メールアドレスは254文字以下である必要があります");
    map.insert(ErrorKey::UserEmailTooLong, user_email_too_long);

    let mut user_email_invalid = HashMap::new();
    user_email_invalid.insert("en", "Invalid email address");
    user_email_invalid.insert("jp", "無効なメールアドレスです");
    map.insert(ErrorKey::UserEmailInvalid, user_email_invalid);

    let mut user_password_empty = HashMap::new();
    user_password_empty.insert("en", "Password cannot be empty");
    user_password_empty.insert("jp", "パスワードは空にできません");
    map.insert(ErrorKey::UserPasswordEmpty, user_password_empty);

    let mut user_password_invalid = HashMap::new();
    user_password_invalid.insert("en", "Password must be a valid SHA-256 hash (lowercase hexadecimal)");
    user_password_invalid.insert("jp", "パスワードは有効なSHA-256ハッシュ（小文字の16進数）である必要があります");
    map.insert(ErrorKey::UserPasswordInvalid, user_password_invalid);

    let mut user_create_failed = HashMap::new();
    user_create_failed.insert("en", "Failed to create user due to database operation failure");
    user_create_failed.insert("jp", "DB操作処理の問題によりユーザーの作成に失敗しました");
    map.insert(ErrorKey::UserCreateFailed, user_create_failed);

    let mut user_get_by_id_failed = HashMap::new();
    user_get_by_id_failed.insert("en", "Failed to get user by ID due to database operation failure");
    user_get_by_id_failed.insert("jp", "DB操作処理の問題によりIDによるユーザーの取得に失敗しました");
    map.insert(ErrorKey::UserGetByIdFailed, user_get_by_id_failed);

    let mut user_get_by_name_failed = HashMap::new();
    user_get_by_name_failed.insert("en", "Failed to get user by name due to database operation failure");
    user_get_by_name_failed.insert("jp", "DB操作処理の問題により名前によるユーザーの取得に失敗しました");
    map.insert(ErrorKey::UserGetByNameFailed, user_get_by_name_failed);

    let mut user_get_all_failed = HashMap::new();
    user_get_all_failed.insert("en", "Failed to get all users due to database operation failure");
    user_get_all_failed.insert("jp", "DB操作処理の問題によりすべてのユーザーの取得に失敗しました");
    map.insert(ErrorKey::UserGetAllFailed, user_get_all_failed);

    let mut user_update_failed = HashMap::new();
    user_update_failed.insert("en", "Failed to update user due to database operation failure");
    user_update_failed.insert("jp", "DB操作処理の問題によりユーザーの更新に失敗しました");
    map.insert(ErrorKey::UserUpdateFailed, user_update_failed);

    let mut user_delete_failed = HashMap::new();
    user_delete_failed.insert("en", "Failed to delete user due to database operation failure");
    user_delete_failed.insert("jp", "DB操作処理の問題によりユーザーの削除に失敗しました");
    map.insert(ErrorKey::UserDeleteFailed, user_delete_failed);

    let mut user_delete_failed_by_id_not_found = HashMap::new();
    user_delete_failed_by_id_not_found.insert("en", "Failed to delete user becouse user does not exist");
    user_delete_failed_by_id_not_found.insert("jp", "存在しないユーザーを削除しようとしました。");
    map.insert(ErrorKey::UserDeleteFailedByIdNotFound, user_delete_failed_by_id_not_found);

    // プロジェクト関連のエラーメッセージ
    let mut project_id_invalid = HashMap::new();
    project_id_invalid.insert("en", "Project ID must be greater than 0");
    project_id_invalid.insert("jp", "プロジェクトIDは0より大きくなければなりません");
    map.insert(ErrorKey::ProjectIdInvalid, project_id_invalid);

    let mut project_name_empty = HashMap::new();
    project_name_empty.insert("en", "Project name cannot be empty");
    project_name_empty.insert("jp", "プロジェクト名は空にできません");
    map.insert(ErrorKey::ProjectNameEmpty, project_name_empty);

    let mut project_name_too_long = HashMap::new();
    project_name_too_long.insert("en", "Project name cannot be longer than 128 characters");
    project_name_too_long.insert("jp", "プロジェクト名は128文字以下である必要があります");
    map.insert(ErrorKey::ProjectNameTooLong, project_name_too_long);

    let mut project_create_failed = HashMap::new();
    project_create_failed.insert("en", "Failed to create project due to database operation failure");
    project_create_failed.insert("jp", "DB操作処理の問題によりプロジェクトの作成に失敗しました");
    map.insert(ErrorKey::ProjectCreateFailed, project_create_failed);

    let mut project_get_by_id_failed = HashMap::new();
    project_get_by_id_failed.insert("en", "Failed to get project by ID due to database operation failure");
    project_get_by_id_failed.insert("jp", "DB操作処理の問題によりIDによるプロジェクトの取得に失敗しました");
    map.insert(ErrorKey::ProjectGetByIdFailed, project_get_by_id_failed);

    let mut project_get_by_name_failed = HashMap::new();
    project_get_by_name_failed.insert("en", "Failed to get project by name due to database operation failure");
    project_get_by_name_failed.insert("jp", "DB操作処理の問題により名前によるプロジェクトの取得に失敗しました");
    map.insert(ErrorKey::ProjectGetByNameFailed, project_get_by_name_failed);

    let mut project_get_all_failed = HashMap::new();
    project_get_all_failed.insert("en", "Failed to get all projects due to database operation failure");
    project_get_all_failed.insert("jp", "DB操作処理の問題によりすべてのプロジェクトの取得に失敗しました");
    map.insert(ErrorKey::ProjectGetAllFailed, project_get_all_failed);

    let mut project_update_failed = HashMap::new();
    project_update_failed.insert("en", "Failed to update project due to database operation failure");
    project_update_failed.insert("jp", "DB操作処理の問題によりプロジェクトの更新に失敗しました");
    map.insert(ErrorKey::ProjectUpdateFailed, project_update_failed);

    let mut project_delete_failed = HashMap::new();
    project_delete_failed.insert("en", "Failed to delete project due to database operation failure");
    project_delete_failed.insert("jp", "DB操作処理の問題によりプロジェクトの削除に失敗しました");
    map.insert(ErrorKey::ProjectDeleteFailed, project_delete_failed);

    let mut project_delete_failed_by_id_not_found = HashMap::new();
    project_delete_failed_by_id_not_found.insert("en", "Failed to delete project becouse project does not exist");
    project_delete_failed_by_id_not_found.insert("jp", "存在しないプロジェクトを削除しようとしました。");
    map.insert(ErrorKey::ProjectDeleteFailedByIdNotFound, project_delete_failed_by_id_not_found);

    // タスク関連のエラーメッセージ
    let mut task_id_invalid = HashMap::new();
    task_id_invalid.insert("en", "Task ID must be greater than 0");
    task_id_invalid.insert("jp", "タスクIDは0より大きくなければなりません");
    map.insert(ErrorKey::TaskIdInvalid, task_id_invalid);

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
    task_description_too_long.insert("en", "Task description cannot be longer than 1024 characters");
    task_description_too_long.insert("jp", "タスクの説明は1024文字以下である必要があります");
    map.insert(ErrorKey::TaskDescriptionTooLong, task_description_too_long);

    let mut task_timestamp_invalid = HashMap::new();
    task_timestamp_invalid.insert("en", "Timestamp must be greater than 0");
    task_timestamp_invalid.insert("jp", "タイムスタンプは0より大きくなければなりません");
    map.insert(ErrorKey::TaskTimestampInvalid, task_timestamp_invalid);

    let mut task_timestamp_or_none_invalid = HashMap::new();
    task_timestamp_or_none_invalid.insert("en", "Timestamp must be greater than 0 or None");
    task_timestamp_or_none_invalid.insert("jp", "タイムスタンプは0より大きいか、Noneである必要があります");
    map.insert(ErrorKey::TaskTimestampOrNoneInvalid, task_timestamp_or_none_invalid);

    let mut task_project_id_not_found = HashMap::new();
    task_project_id_not_found.insert("en", "Project not found.");
    task_project_id_not_found.insert("jp", "プロジェクトが見つかりません");
    map.insert(ErrorKey::TaskProjectIdNotFound, task_project_id_not_found);

    let mut task_no_parent_id_on_non_major_task = HashMap::new();
    task_no_parent_id_on_non_major_task.insert("en", "Parent ID is required for non-major tasks");
    task_no_parent_id_on_non_major_task.insert("jp", "大項目タスク以外のタスクには親タスクIDが必要です");
    map.insert(ErrorKey::TaskNoParentIdOnNonMajorTask, task_no_parent_id_on_non_major_task);

    let mut task_parent_id_not_found = HashMap::new();
    task_parent_id_not_found.insert("en", "Parent task not found");
    task_parent_id_not_found.insert("jp", "親タスクが見つかりません");
    map.insert(ErrorKey::TaskParentIdNotFound, task_parent_id_not_found);

    let mut task_parent_level_invalid = HashMap::new();
    task_parent_level_invalid.insert("en", "Parent task level is not one level higher");
    task_parent_level_invalid.insert("jp", "親タスクのレベルが1つ上ではありません");
    map.insert(ErrorKey::TaskParentLevelInvalid, task_parent_level_invalid);

    let mut task_parent_id_cannot_be_same_as_task_id = HashMap::new();
    task_parent_id_cannot_be_same_as_task_id.insert("en", "Parent ID cannot be the same as the task ID");
    task_parent_id_cannot_be_same_as_task_id.insert("jp", "親タスクIDはタスクIDと同じにできません");
    map.insert(ErrorKey::TaskParentIdCannotBeSameAsTaskId, task_parent_id_cannot_be_same_as_task_id);

    let mut task_create_failed = HashMap::new();
    task_create_failed.insert("en", "Failed to create task due to database operation failure");
    task_create_failed.insert("jp", "DB操作処理の問題によりタスクの作成に失敗しました");
    map.insert(ErrorKey::TaskCreateFailed, task_create_failed);

    let mut task_get_by_id_failed = HashMap::new();
    task_get_by_id_failed.insert("en", "Failed to get task by ID due to database operation failure");
    task_get_by_id_failed.insert("jp", "DB操作処理の問題によりIDによるタスクの取得に失敗しました");
    map.insert(ErrorKey::TaskGetByIdFailed, task_get_by_id_failed);

    let mut task_get_all_failed = HashMap::new();
    task_get_all_failed.insert("en", "Failed to get all tasks due to database operation failure");
    task_get_all_failed.insert("jp", "DB操作処理の問題によりすべてのタスクの取得に失敗しました");
    map.insert(ErrorKey::TaskGetAllFailed, task_get_all_failed);

    let mut task_get_by_filter_failed = HashMap::new();
    task_get_by_filter_failed.insert("en", "Failed to get tasks by filter due to database operation failure");
    task_get_by_filter_failed.insert("jp", "DB操作処理の問題によりフィルターによるタスクの取得に失敗しました");
    map.insert(ErrorKey::TaskGetByFilterFailed, task_get_by_filter_failed);

    let mut task_update_failed = HashMap::new();
    task_update_failed.insert("en", "Failed to update task due to database operation failure");
    task_update_failed.insert("jp", "DB操作処理の問題によりタスクの更新に失敗しました");
    map.insert(ErrorKey::TaskUpdateFailed, task_update_failed);

    let mut task_delete_failed = HashMap::new();
    task_delete_failed.insert("en", "Failed to delete task due to database operation failure");
    task_delete_failed.insert("jp", "DB操作処理の問題によりタスクの削除に失敗しました");
    map.insert(ErrorKey::TaskDeleteFailed, task_delete_failed);

    let mut task_delete_failed_by_id_not_found = HashMap::new();
    task_delete_failed_by_id_not_found.insert("en", "Failed to delete task becouse task does not exist");
    task_delete_failed_by_id_not_found.insert("jp", "存在しないタスクを削除しようとしました。");
    map.insert(ErrorKey::TaskDeleteFailedByIdNotFound, task_delete_failed_by_id_not_found);

    // ユーザー割り当て関連のエラーメッセージ
    let mut user_assign_create_failed = HashMap::new();
    user_assign_create_failed.insert("en", "Failed to create user assign due to database operation failure");
    user_assign_create_failed.insert("jp", "DB操作処理の問題によりユーザー割り当ての作成に失敗しました");
    map.insert(ErrorKey::UserAssignCreateFailed, user_assign_create_failed);
    
    let mut user_assign_get_by_id_failed = HashMap::new();
    user_assign_get_by_id_failed.insert("en", "Failed to get user assign by ID due to database operation failure");
    user_assign_get_by_id_failed.insert("jp", "DB操作処理の問題によりIDによるユーザー割り当ての取得に失敗しました");
    map.insert(ErrorKey::UserAssignGetByIdFailed, user_assign_get_by_id_failed);
    
    let mut user_assign_get_by_task_id_failed = HashMap::new();
    user_assign_get_by_task_id_failed.insert("en", "Failed to get user assign by task ID due to database operation failure");
    user_assign_get_by_task_id_failed.insert("jp", "DB操作処理の問題によりタスクIDによるユーザー割り当ての取得に失敗しました");
    map.insert(ErrorKey::UserAssignGetByTaskIdFailed, user_assign_get_by_task_id_failed);
    
    let mut user_assign_get_by_user_id_failed = HashMap::new();
    user_assign_get_by_user_id_failed.insert("en", "Failed to get user assign by user ID due to database operation failure");
    user_assign_get_by_user_id_failed.insert("jp", "DB操作処理の問題によりユーザーIDによるユーザー割り当ての取得に失敗しました");
    map.insert(ErrorKey::UserAssignGetByUserIdFailed, user_assign_get_by_user_id_failed);

    let mut user_assign_get_by_user_id_and_task_id_failed = HashMap::new();
    user_assign_get_by_user_id_and_task_id_failed.insert("en", "Failed to get user assign by user ID and task ID due to database operation failure");
    user_assign_get_by_user_id_and_task_id_failed.insert("jp", "DB操作処理の問題によりユーザーIDとタスクIDによるユーザー割り当ての取得に失敗しました");
    map.insert(ErrorKey::UserAssignGetByUserIdAndTaskIdFailed, user_assign_get_by_user_id_and_task_id_failed);
    
    let mut user_assign_get_all_failed = HashMap::new();
    user_assign_get_all_failed.insert("en", "Failed to get all user assigns due to database operation failure");
    user_assign_get_all_failed.insert("jp", "DB操作処理の問題によりすべてのユーザー割り当ての取得に失敗しました");
    map.insert(ErrorKey::UserAssignGetAllFailed, user_assign_get_all_failed);
    
    let mut user_assign_update_failed = HashMap::new();
    user_assign_update_failed.insert("en", "Failed to update user assign due to database operation failure");
    user_assign_update_failed.insert("jp", "DB操作処理の問題によりユーザー割り当ての更新に失敗しました");
    map.insert(ErrorKey::UserAssignUpdateFailed, user_assign_update_failed);

    let mut user_assign_delete_failed = HashMap::new();
    user_assign_delete_failed.insert("en", "Failed to delete user assign due to database operation failure");
    user_assign_delete_failed.insert("jp", "DB操作処理の問題によりユーザー割り当ての削除に失敗しました");
    map.insert(ErrorKey::UserAssignDeleteFailed, user_assign_delete_failed);

    let mut user_assign_delete_failed_by_id_not_found = HashMap::new();
    user_assign_delete_failed_by_id_not_found.insert("en", "Failed to delete user assign becouse user assign does not exist");
    user_assign_delete_failed_by_id_not_found.insert("jp", "存在しないユーザー割り当てを削除しようとしました。");
    map.insert(ErrorKey::UserAssignDeleteFailedByIdNotFound, user_assign_delete_failed_by_id_not_found);

    let mut user_assign_user_id_invalid = HashMap::new();
    user_assign_user_id_invalid.insert("en", "User ID must be greater than 0");
    user_assign_user_id_invalid.insert("jp", "ユーザーIDは0より大きくなければなりません");
    map.insert(ErrorKey::UserAssignUserIdInvalid, user_assign_user_id_invalid);

    let mut user_assign_task_id_invalid = HashMap::new();
    user_assign_task_id_invalid.insert("en", "Task ID must be greater than 0");
    user_assign_task_id_invalid.insert("jp", "タスクIDは0より大きくなければなりません");
    map.insert(ErrorKey::UserAssignTaskIdInvalid, user_assign_task_id_invalid);

    let mut user_assign_user_id_not_found = HashMap::new();
    user_assign_user_id_not_found.insert("en", "The user assign to task does not exist");
    user_assign_user_id_not_found.insert("jp", "存在しないユーザーをタスクに割り当てしようとしました。");
    map.insert(ErrorKey::UserAssignUserIdNotFound, user_assign_user_id_not_found);

    let mut user_assign_task_id_not_found = HashMap::new();
    user_assign_task_id_not_found.insert("en", "The task assign to user does not exist");
    user_assign_task_id_not_found.insert("jp", "存在しないタスクをユーザーに割り当てしようとしました。");
    map.insert(ErrorKey::UserAssignTaskIdNotFound, user_assign_task_id_not_found);

    let mut user_assign_to_not_max_level_task = HashMap::new();
    user_assign_to_not_max_level_task.insert("en", "The user can only be assigned to tasks at the maximum level");
    user_assign_to_not_max_level_task.insert("jp", "ユーザーは最も詳細な階層のタスクにしか割り当てられません");
    map.insert(ErrorKey::UserAssignToNotMaxLevelTask, user_assign_to_not_max_level_task);

    let mut user_assign_same_user_assign_exists = HashMap::new();
    user_assign_same_user_assign_exists.insert("en", "The user is already assigned to the task");
    user_assign_same_user_assign_exists.insert("jp", "すでにユーザーはタスクに割り当てられています");
    map.insert(ErrorKey::UserAssignSameUserAssignExists, user_assign_same_user_assign_exists);

    map
});

pub fn get_error_message(key: ErrorKey, error_info: String) -> String {
    let lang = get_lang();

    let msg = ERROR_MESSAGES
        .get(&key)
        .and_then(|messages| messages.get(lang))
        .unwrap_or_else(|| {
            // デフォルトは英語
            ERROR_MESSAGES
                .get(&key)
                .and_then(|messages| messages.get("en"))
                .unwrap_or(&"Unknown error")
        });

    let error_code = key.to_string();
    format!("[{}] {}: ({})", error_code, msg, error_info)
}

pub fn get_lang() -> &'static str {
    GLOBAL_LANG.as_ref()
}