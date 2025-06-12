use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use crate::errors::message_def::user::add_user_error_messages;
use crate::errors::message_def::project::add_project_error_messages;
use crate::errors::message_def::task::add_task_error_messages;
use crate::errors::message_def::user_assign::add_user_assign_error_messages;
use crate::errors::message_def::comment::add_comment_error_messages;
use crate::errors::message_def::user_handler::add_user_handler_error_messages;
use crate::errors::message_def::project_handler::add_project_handler_error_messages;
use crate::errors::message_def::task_handler::add_task_handler_error_messages;
use crate::errors::message_def::user_assign_handler::add_user_assign_handler_error_messages;
use crate::errors::message_def::task_user::add_task_user_error_messages;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKey {
    // ユーザー関連のエラー
    UserIdInvalid,
    UserIdMustBeNone,
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
    UserGetUsersCountFailed,
    UserGetByIdNotFound,
    UserGetByNameNotFound,
    UserGetUsersPaginationNotFound,

    // プロジェクト関連のエラー
    ProjectIdInvalid,
    ProjectIdMustBeNone,
    ProjectNameEmpty,
    ProjectNameTooLong,
    ProjectCreateFailed,
    ProjectGetByIdFailed,
    ProjectGetByNameFailed,
    ProjectGetAllFailed,
    ProjectGetByIdNotFound,
    ProjectGetByNameNotFound,
    ProjectUpdateFailed,
    ProjectDeleteFailed,
    ProjectDeleteFailedByIdNotFound,
    ProjectGetProjectsCountFailed,
    ProjectGetPaginationNotFound,

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
    TaskGetCountFailed,
    TaskGetPaginationNotFound,
    TaskGetByIdNotFound,

    // ユーザー割り当て関連のエラー
    UserAssignIdInvalid,
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
    UserAssignGetByIdNotFound,
    UserAssignGetPaginationNotFound,
    UserAssignGetByUserIdAndTaskIdNotFound,
    UserAssignGetUserAssignsCountFailed,

    // コメント関連のエラー
    CommentCreateFailed,
    CommentGetByIdFailed,
    CommentGetByTaskIdFailed,
    CommentGetByUserIdFailed,
    CommentGetByUserIdAndTaskIdFailed,
    CommentGetAllFailed,
    CommentUpdateFailed,
    CommentDeleteFailed,
    CommentDeleteFailedByIdNotFound,
    CommentUserIdInvalid,
    CommentTaskIdInvalid,
    CommentUserIdNotFound,
    CommentTaskIdNotFound,
    CommentContentEmpty,
    CommentToNotMaxLevelTask,
    CommentContentTooLong,
    CommentGetByIdNotFound,
    CommentGetPaginationNotFound,
    CommentGetCountFailed,

    // タスク+ユーザー関連のエラー
    TaskUserGetAllFailed,
    TaskUserGetByFilterFailed,
    TaskUserGetPaginationNotFound,

    // リポジトリ共通エラー
    NoPageSpecified,
    NoPageSizeSpecified,
    InvalidPagination,
    PageSizeTooLarge,

    // ユーザーハンドラ関連のエラー
    UserHandlerGetUsersInvalidPage,
    UserHandlerGetUsersInvalidTarget,
    UserHandlerGetUsersNoNameSpecified,
    UserHandlerGetUsersNoIdSpecified,
    UserHandlerPathAndBodyIdMismatch,
    UserHandlerInvalidJsonPost,
    UserHandlerInvalidQuery,
    UserHandlerInvalidPath,

    // プロジェクトハンドラ関連のエラー
    ProjectHandlerGetProjectsInvalidPage,
    ProjectHandlerGetProjectsInvalidTarget,
    ProjectHandlerGetProjectsNoNameSpecified,
    ProjectHandlerGetProjectsNoIdSpecified,
    ProjectHandlerPathAndBodyIdMismatch,
    ProjectHandlerInvalidJsonPost,
    ProjectHandlerInvalidQuery,
    ProjectHandlerInvalidPath,

    // タスクハンドラ関連のエラー
    TaskHandlerGetTasksInvalidPage,
    TaskHandlerGetTasksInvalidTarget,
    TaskHandlerGetTasksNoIdSpecified,
    TaskHandlerInvalidJsonPost,
    TaskHandlerPathAndBodyIdMismatch,
    TaskHandlerInvalidQuery,
    TaskHandlerInvalidPath,

    // ユーザー割り当てハンドラ関連のエラー
    UserAssignHandlerGetUserAssignsInvalidPage,
    UserAssignHandlerGetUserAssignsInvalidTarget,
    UserAssignHandlerGetUserAssignsNoIdSpecified,
    UserAssignHandlerGetUserAssignsNoUserIdSpecified,
    UserAssignHandlerGetUserAssignsNoTaskIdSpecified,
    UserAssignHandlerInvalidJsonPost,
    UserAssignHandlerPathAndBodyIdMismatch,
    UserAssignHandlerInvalidQuery,
    UserAssignHandlerInvalidPath,
}

impl fmt::Display for ErrorKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ユーザー関連のエラー
            ErrorKey::UserIdInvalid => write!(f, "UserIdInvalid"),
            ErrorKey::UserIdMustBeNone => write!(f, "UserIdMustBeNone"),
            ErrorKey::UserNameEmpty => write!(f, "UserNameEmpty"),
            ErrorKey::UserNameTooLong => write!(f, "UserNameTooLong"),
            ErrorKey::UserNameContainsInvalidCharacters => {
                write!(f, "UserNameContainsInvalidCharacters")
            }
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
            ErrorKey::UserGetUsersCountFailed => write!(f, "UserGetUsersCountFailed"),
            ErrorKey::UserGetByIdNotFound => write!(f, "UserGetByIdNotFound"),
            ErrorKey::UserGetByNameNotFound => write!(f, "UserGetByNameNotFound"),
            ErrorKey::UserGetUsersPaginationNotFound => write!(f, "UserGetUsersPaginationNotFound"),

            // プロジェクト関連のエラー
            ErrorKey::ProjectIdInvalid => write!(f, "ProjectIdInvalid"),
            ErrorKey::ProjectIdMustBeNone => write!(f, "ProjectIdMustBeNone"),
            ErrorKey::ProjectGetByIdNotFound => write!(f, "ProjectGetByIdNotFound"),
            ErrorKey::ProjectGetByNameNotFound => write!(f, "ProjectGetByNameNotFound"),
            ErrorKey::ProjectNameEmpty => write!(f, "ProjectNameEmpty"),
            ErrorKey::ProjectNameTooLong => write!(f, "ProjectNameTooLong"),
            ErrorKey::ProjectCreateFailed => write!(f, "ProjectCreateFailed"),
            ErrorKey::ProjectGetByIdFailed => write!(f, "ProjectGetByIdFailed"),
            ErrorKey::ProjectGetByNameFailed => write!(f, "ProjectGetByNameFailed"),
            ErrorKey::ProjectGetAllFailed => write!(f, "ProjectGetAllFailed"),
            ErrorKey::ProjectUpdateFailed => write!(f, "ProjectUpdateFailed"),
            ErrorKey::ProjectDeleteFailed => write!(f, "ProjectDeleteFailed"),
            ErrorKey::ProjectDeleteFailedByIdNotFound => {
                write!(f, "ProjectDeleteFailedByIdNotFound")
            }
            ErrorKey::ProjectGetProjectsCountFailed => write!(f, "ProjectGetProjectsCountFailed"),
            ErrorKey::ProjectGetPaginationNotFound => write!(f, "ProjectGetPaginationNotFound"),

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
            ErrorKey::TaskParentIdCannotBeSameAsTaskId => {
                write!(f, "TaskParentIdCannotBeSameAsTaskId")
            }
            ErrorKey::TaskCreateFailed => write!(f, "TaskCreateFailed"),
            ErrorKey::TaskGetByIdFailed => write!(f, "TaskGetByIdFailed"),
            ErrorKey::TaskGetAllFailed => write!(f, "TaskGetAllFailed"),
            ErrorKey::TaskGetByFilterFailed => write!(f, "TaskGetByFilterFailed"),
            ErrorKey::TaskUpdateFailed => write!(f, "TaskUpdateFailed"),
            ErrorKey::TaskDeleteFailed => write!(f, "TaskDeleteFailed"),
            ErrorKey::TaskDeleteFailedByIdNotFound => write!(f, "TaskDeleteFailedByIdNotFound"),
            ErrorKey::TaskGetCountFailed => write!(f, "TaskGetCountFailed"),
            ErrorKey::TaskGetPaginationNotFound => write!(f, "TaskGetPaginationNotFound"),
            ErrorKey::TaskGetByIdNotFound => write!(f, "TaskGetByIdNotFound"),  

            // ユーザー割り当て関連のエラー
            ErrorKey::UserAssignIdInvalid => write!(f, "UserAssignIdInvalid"),
            ErrorKey::UserAssignCreateFailed => write!(f, "UserAssignCreateFailed"),
            ErrorKey::UserAssignGetByIdFailed => write!(f, "UserAssignGetByIdFailed"),
            ErrorKey::UserAssignGetByTaskIdFailed => write!(f, "UserAssignGetByTaskIdFailed"),
            ErrorKey::UserAssignGetAllFailed => write!(f, "UserAssignGetAllFailed"),
            ErrorKey::UserAssignUpdateFailed => write!(f, "UserAssignUpdateFailed"),
            ErrorKey::UserAssignDeleteFailed => write!(f, "UserAssignDeleteFailed"),
            ErrorKey::UserAssignDeleteFailedByIdNotFound => {
                write!(f, "UserAssignDeleteFailedByIdNotFound")
            }
            ErrorKey::UserAssignUserIdInvalid => write!(f, "UserAssignUserIdInvalid"),
            ErrorKey::UserAssignTaskIdInvalid => write!(f, "UserAssignTaskIdInvalid"),
            ErrorKey::UserAssignUserIdNotFound => write!(f, "UserAssignUserIdNotFound"),
            ErrorKey::UserAssignTaskIdNotFound => write!(f, "UserAssignTaskIdNotFound"),
            ErrorKey::UserAssignToNotMaxLevelTask => write!(f, "UserAssignToNotMaxLevelTask"),
            ErrorKey::UserAssignSameUserAssignExists => write!(f, "UserAssignSameUserAssignExists"),
            ErrorKey::UserAssignGetByUserIdFailed => write!(f, "UserAssignGetByUserIdFailed"),
            ErrorKey::UserAssignGetByUserIdAndTaskIdFailed => {
                write!(f, "UserAssignGetByUserIdAndTaskIdFailed")
            }
            ErrorKey::UserAssignGetByIdNotFound => write!(f, "UserAssignGetByIdNotFound"),
            ErrorKey::UserAssignGetPaginationNotFound => write!(f, "UserAssignGetPaginationNotFound"),
            ErrorKey::UserAssignGetByUserIdAndTaskIdNotFound => {
                write!(f, "UserAssignGetByUserIdAndTaskIdNotFound")
            }
            ErrorKey::UserAssignGetUserAssignsCountFailed => {
                write!(f, "UserAssignGetUserAssignsCountFailed")
            }

            // コメント関連のエラー
            ErrorKey::CommentCreateFailed => write!(f, "CommentCreateFailed"),
            ErrorKey::CommentGetByIdFailed => write!(f, "CommentGetByIdFailed"),
            ErrorKey::CommentGetByTaskIdFailed => write!(f, "CommentGetByTaskIdFailed"),
            ErrorKey::CommentGetByUserIdFailed => write!(f, "CommentGetByUserIdFailed"),
            ErrorKey::CommentGetByUserIdAndTaskIdFailed => {
                write!(f, "CommentGetByUserIdAndTaskIdFailed")
            }
            ErrorKey::CommentGetAllFailed => write!(f, "CommentGetAllFailed"),
            ErrorKey::CommentUpdateFailed => write!(f, "CommentUpdateFailed"),
            ErrorKey::CommentDeleteFailed => write!(f, "CommentDeleteFailed"),
            ErrorKey::CommentDeleteFailedByIdNotFound => {
                write!(f, "CommentDeleteFailedByIdNotFound")
            }
            ErrorKey::CommentUserIdInvalid => write!(f, "CommentUserIdInvalid"),
            ErrorKey::CommentTaskIdInvalid => write!(f, "CommentTaskIdInvalid"),
            ErrorKey::CommentUserIdNotFound => write!(f, "CommentUserIdNotFound"),
            ErrorKey::CommentTaskIdNotFound => write!(f, "CommentTaskIdNotFound"),
            ErrorKey::CommentContentEmpty => write!(f, "CommentContentEmpty"),
            ErrorKey::CommentContentTooLong => write!(f, "CommentContentTooLong"),
            ErrorKey::CommentToNotMaxLevelTask => write!(f, "CommentToNotMaxLevelTask"),
            ErrorKey::CommentGetByIdNotFound => write!(f, "CommentGetByIdNotFound"),
            ErrorKey::CommentGetPaginationNotFound => write!(f, "CommentGetPaginationNotFound"),    
            ErrorKey::CommentGetCountFailed => write!(f, "CommentGetCountFailed"),

            // リポジトリ共通エラー
            ErrorKey::NoPageSpecified => write!(f, "NoPageSpecified"),
            ErrorKey::NoPageSizeSpecified => write!(f, "NoPageSizeSpecified"),
            ErrorKey::InvalidPagination => write!(f, "InvalidPagination"),
            ErrorKey::PageSizeTooLarge => write!(f, "PageSizeTooLarge"),

            // タスク+ユーザー関連のエラー
            ErrorKey::TaskUserGetAllFailed => write!(f, "TaskUserGetAllFailed"),
            ErrorKey::TaskUserGetByFilterFailed => write!(f, "TaskUserGetByFilterFailed"),
            ErrorKey::TaskUserGetPaginationNotFound => write!(f, "TaskUserGetPaginationNotFound"),

            // ユーザーハンドラ関連のエラー
            ErrorKey::UserHandlerGetUsersInvalidPage => write!(f, "UserHandlerGetUsersInvalidPage"),
            ErrorKey::UserHandlerGetUsersInvalidTarget => write!(f, "UserHandlerGetUsersInvalidTarget"),
            ErrorKey::UserHandlerGetUsersNoNameSpecified => write!(f, "UserHandlerGetUsersNoNameSpecified"),
            ErrorKey::UserHandlerGetUsersNoIdSpecified => write!(f, "UserHandlerGetUsersNoIdSpecified"),
            ErrorKey::UserHandlerPathAndBodyIdMismatch => write!(f, "UserHandlerPathAndBodyIdMismatch"),
            ErrorKey::UserHandlerInvalidJsonPost => write!(f, "UserHandlerInvalidJsonPost"),
            ErrorKey::UserHandlerInvalidQuery => write!(f, "UserHandlerInvalidQuery"),
            ErrorKey::UserHandlerInvalidPath => write!(f, "UserHandlerInvalidPath"),

            // プロジェクトハンドラ関連のエラー
            ErrorKey::ProjectHandlerGetProjectsInvalidPage => write!(f, "ProjectHandlerGetProjectsInvalidPage"),
            ErrorKey::ProjectHandlerGetProjectsInvalidTarget => write!(f, "ProjectHandlerGetProjectsInvalidTarget"),
            ErrorKey::ProjectHandlerGetProjectsNoNameSpecified => write!(f, "ProjectHandlerGetProjectsNoNameSpecified"),
            ErrorKey::ProjectHandlerGetProjectsNoIdSpecified => write!(f, "ProjectHandlerGetProjectsNoIdSpecified"),
            ErrorKey::ProjectHandlerPathAndBodyIdMismatch => write!(f, "ProjectHandlerPathAndBodyIdMismatch"),
            ErrorKey::ProjectHandlerInvalidJsonPost => write!(f, "ProjectHandlerInvalidJsonPost"),
            ErrorKey::ProjectHandlerInvalidQuery => write!(f, "ProjectHandlerInvalidQuery"),
            ErrorKey::ProjectHandlerInvalidPath => write!(f, "ProjectHandlerInvalidPath"),

            // タスクハンドラ関連のエラー
            ErrorKey::TaskHandlerGetTasksInvalidPage => write!(f, "TaskHandlerGetTasksInvalidPage"),
            ErrorKey::TaskHandlerGetTasksInvalidTarget => write!(f, "TaskHandlerGetTasksInvalidTarget"),
            ErrorKey::TaskHandlerGetTasksNoIdSpecified => write!(f, "TaskHandlerGetTasksNoIdSpecified"),
            ErrorKey::TaskHandlerInvalidJsonPost => write!(f, "TaskHandlerInvalidJsonPost"),
            ErrorKey::TaskHandlerPathAndBodyIdMismatch => write!(f, "TaskHandlerPathAndBodyIdMismatch"),
            ErrorKey::TaskHandlerInvalidQuery => write!(f, "TaskHandlerInvalidQuery"),
            ErrorKey::TaskHandlerInvalidPath => write!(f, "TaskHandlerInvalidPath"),

            // ユーザー割り当てハンドラ関連のエラー
            ErrorKey::UserAssignHandlerGetUserAssignsInvalidPage => write!(f, "UserAssignHandlerGetUserAssignsInvalidPage"),
            ErrorKey::UserAssignHandlerGetUserAssignsInvalidTarget => write!(f, "UserAssignHandlerGetUserAssignsInvalidTarget"),
            ErrorKey::UserAssignHandlerGetUserAssignsNoIdSpecified => write!(f, "UserAssignHandlerGetUserAssignsNoIdSpecified"),
            ErrorKey::UserAssignHandlerGetUserAssignsNoUserIdSpecified => write!(f, "UserAssignHandlerGetUserAssignsNoUserIdSpecified"),
            ErrorKey::UserAssignHandlerGetUserAssignsNoTaskIdSpecified => write!(f, "UserAssignHandlerGetUserAssignsNoTaskIdSpecified"),
            ErrorKey::UserAssignHandlerInvalidJsonPost => write!(f, "UserAssignHandlerInvalidJsonPost"),
            ErrorKey::UserAssignHandlerPathAndBodyIdMismatch => write!(f, "UserAssignHandlerPathAndBodyIdMismatch"),
            ErrorKey::UserAssignHandlerInvalidQuery => write!(f, "UserAssignHandlerInvalidQuery"),
            ErrorKey::UserAssignHandlerInvalidPath => write!(f, "UserAssignHandlerInvalidPath"),
        }
    }
}

static GLOBAL_LANG: Lazy<&'static str> = Lazy::new(|| {
    //[TODO] Configから取得するようにする
    "en"
});

static ERROR_MESSAGES: Lazy<HashMap<ErrorKey, HashMap<&'static str, &'static str>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();

        add_user_error_messages(&mut map);
        add_project_error_messages(&mut map);
        add_task_error_messages(&mut map);
        add_user_assign_error_messages(&mut map);
        add_comment_error_messages(&mut map);
        add_user_handler_error_messages(&mut map);
        add_project_handler_error_messages(&mut map);
        add_task_handler_error_messages(&mut map);
        add_user_assign_handler_error_messages(&mut map);
        add_task_user_error_messages(&mut map);

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