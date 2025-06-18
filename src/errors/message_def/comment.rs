use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_comment_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    // コメント関連のエラーメッセージ
    let mut comment_id_invalid = HashMap::new();
    comment_id_invalid.insert("en", "Comment ID must be greater than 0");
    comment_id_invalid.insert("jp", "コメントIDは0より大きくなければなりません");
    map.insert(ErrorKey::CommentIdInvalid, comment_id_invalid);

    let mut comment_id_must_be_none = HashMap::new();
    comment_id_must_be_none.insert("en", "Comment ID must be none");
    comment_id_must_be_none.insert("jp", "コメントIDはnullでなければなりません");
    map.insert(ErrorKey::CommentIdMustBeNone, comment_id_must_be_none);

    let mut comment_create_failed = HashMap::new();
    comment_create_failed.insert(
        "en",
        "Failed to create comment due to database operation failure",
    );
    comment_create_failed.insert("jp", "DB操作処理の問題によりコメントの作成に失敗しました");
    map.insert(ErrorKey::CommentCreateFailed, comment_create_failed);

    let mut comment_get_by_id_failed = HashMap::new();
    comment_get_by_id_failed.insert(
        "en",
        "Failed to get comment by ID due to database operation failure",
    );
    comment_get_by_id_failed.insert(
        "jp",
        "DB操作処理の問題によりIDによるコメントの取得に失敗しました",
    );
    map.insert(ErrorKey::CommentGetByIdFailed, comment_get_by_id_failed);

    let mut comment_get_by_task_id_failed = HashMap::new();
    comment_get_by_task_id_failed.insert(
        "en",
        "Failed to get comment by task ID due to database operation failure",
    );
    comment_get_by_task_id_failed.insert(
        "jp",
        "DB操作処理の問題によりタスクIDによるコメントの取得に失敗しました",
    );
    map.insert(
        ErrorKey::CommentGetByTaskIdFailed,
        comment_get_by_task_id_failed,
    );

    let mut comment_get_by_user_id_failed = HashMap::new();
    comment_get_by_user_id_failed.insert(
        "en",
        "Failed to get comment by user ID due to database operation failure",
    );
    comment_get_by_user_id_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザーIDによるコメントの取得に失敗しました",
    );
    map.insert(
        ErrorKey::CommentGetByUserIdFailed,
        comment_get_by_user_id_failed,
    );

    let mut comment_get_by_user_id_and_task_id_failed = HashMap::new();
    comment_get_by_user_id_and_task_id_failed.insert(
        "en",
        "Failed to get comment by user ID and task ID due to database operation failure",
    );
    comment_get_by_user_id_and_task_id_failed.insert(
        "jp",
        "DB操作処理の問題によりユーザーIDとタスクIDによるコメントの取得に失敗しました",
    );
    map.insert(
        ErrorKey::CommentGetByUserIdAndTaskIdFailed,
        comment_get_by_user_id_and_task_id_failed,
    );

    let mut comment_get_all_failed = HashMap::new();
    comment_get_all_failed.insert(
        "en",
        "Failed to get all comments due to database operation failure",
    );
    comment_get_all_failed.insert(
        "jp",
        "DB操作処理の問題によりすべてのコメントの取得に失敗しました",
    );
    map.insert(ErrorKey::CommentGetAllFailed, comment_get_all_failed);

    let mut comment_update_failed = HashMap::new();
    comment_update_failed.insert(
        "en",
        "Failed to update comment due to database operation failure",
    );
    comment_update_failed.insert("jp", "DB操作処理の問題によりコメントの更新に失敗しました");
    map.insert(ErrorKey::CommentUpdateFailed, comment_update_failed);

    let mut comment_delete_failed = HashMap::new();
    comment_delete_failed.insert(
        "en",
        "Failed to delete comment due to database operation failure",
    );
    comment_delete_failed.insert("jp", "DB操作処理の問題によりコメントの削除に失敗しました");
    map.insert(ErrorKey::CommentDeleteFailed, comment_delete_failed);

    let mut comment_delete_failed_by_id_not_found = HashMap::new();
    comment_delete_failed_by_id_not_found.insert(
        "en",
        "Failed to delete comment becouse comment does not exist",
    );
    comment_delete_failed_by_id_not_found
        .insert("jp", "存在しないコメントを削除しようとしました。");
    map.insert(
        ErrorKey::CommentDeleteFailedByIdNotFound,
        comment_delete_failed_by_id_not_found,
    );

    let mut comment_user_id_invalid = HashMap::new();
    comment_user_id_invalid.insert("en", "User ID must be greater than 0");
    comment_user_id_invalid.insert("jp", "ユーザーIDは0より大きくなければなりません");
    map.insert(ErrorKey::CommentUserIdInvalid, comment_user_id_invalid);

    let mut comment_task_id_invalid = HashMap::new();
    comment_task_id_invalid.insert("en", "Task ID must be greater than 0");
    comment_task_id_invalid.insert("jp", "タスクIDは0より大きくなければなりません");
    map.insert(ErrorKey::CommentTaskIdInvalid, comment_task_id_invalid);

    let mut comment_id_not_found = HashMap::new();
    comment_id_not_found.insert("en", "Comment not found");
    comment_id_not_found.insert("jp", "コメントが見つかりません");
    map.insert(ErrorKey::CommentIdNotFound, comment_id_not_found);

    let mut comment_user_id_not_found = HashMap::new();
    comment_user_id_not_found.insert("en", "The user comment does not exist");
    comment_user_id_not_found.insert("jp", "存在しないユーザーのコメントです");
    map.insert(ErrorKey::CommentUserIdNotFound, comment_user_id_not_found);

    let mut comment_task_id_not_found = HashMap::new();
    comment_task_id_not_found.insert("en", "The task comment does not exist");
    comment_task_id_not_found.insert("jp", "存在しないタスクのコメントです");
    map.insert(ErrorKey::CommentTaskIdNotFound, comment_task_id_not_found);

    let mut comment_content_empty = HashMap::new();
    comment_content_empty.insert("en", "Comment content cannot be empty");
    comment_content_empty.insert("jp", "コメント内容は空にできません");
    map.insert(ErrorKey::CommentContentEmpty, comment_content_empty);

    let mut comment_content_too_long = HashMap::new();
    comment_content_too_long.insert(
        "en",
        "Comment content cannot be longer than 1024 characters",
    );
    comment_content_too_long.insert("jp", "コメント内容は2024文字以下である必要があります");
    map.insert(ErrorKey::CommentContentTooLong, comment_content_too_long);

    let mut comment_to_not_max_level_task = HashMap::new();
    comment_to_not_max_level_task.insert(
        "en",
        "The comment can only be added to the task at the maximum level",
    );
    comment_to_not_max_level_task
        .insert("jp", "コメントは最も詳細な階層のタスクにしか追加できません");
    map.insert(
        ErrorKey::CommentToNotMaxLevelTask,
        comment_to_not_max_level_task,
    );

    let mut comment_get_by_id_not_found = HashMap::new();
    comment_get_by_id_not_found.insert("en", "Comment not found");
    comment_get_by_id_not_found.insert("jp", "コメントが見つかりません");
    map.insert(
        ErrorKey::CommentGetByIdNotFound,
        comment_get_by_id_not_found,
    );

    let mut comment_get_pagination_not_found = HashMap::new();
    comment_get_pagination_not_found.insert("en", "No comments found in the specified page");
    comment_get_pagination_not_found.insert("jp", "指定ページ内にコメントが存在しません。");
    map.insert(
        ErrorKey::CommentGetPaginationNotFound,
        comment_get_pagination_not_found,
    );

    let mut comment_get_count_failed = HashMap::new();
    comment_get_count_failed.insert(
        "en",
        "Failed to get comments count due to database operation failure",
    );
    comment_get_count_failed.insert(
        "jp",
        "DB操作処理の問題によりコメントの数の取得に失敗しました",
    );
    map.insert(ErrorKey::CommentGetCountFailed, comment_get_count_failed);
}
