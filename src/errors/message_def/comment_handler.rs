use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_comment_handler_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    // コメントハンドラ関連のエラーメッセージ
    let mut comment_handler_get_comment_invalid_page = HashMap::new();
    comment_handler_get_comment_invalid_page.insert(
        "en",
        "Invalid page or page size. Page must be greater than 0, and page size must be greater than 0 and less than 101",
    );
    comment_handler_get_comment_invalid_page.insert(
        "jp", 
        "ページまたはページサイズが指定されていません。また、ページは1以上、ページサイズは1 <= 100である必要があります");
    map.insert(
        ErrorKey::CommentHandlerGetCommentInvalidPage,
        comment_handler_get_comment_invalid_page,
    );

    let mut comment_handler_get_comment_invalid_target = HashMap::new();
    comment_handler_get_comment_invalid_target.insert(
        "en",
        "Invalid target. Target must be 'all', 'id', 'task_id', or 'user_id'",
    );
    comment_handler_get_comment_invalid_target.insert(
        "jp", 
        "ターゲットが指定されていません。ターゲットは'all', 'id', 'task_id', または'user_id'である必要があります");
    map.insert(
        ErrorKey::CommentHandlerGetCommentInvalidTarget,
        comment_handler_get_comment_invalid_target,
    );

    let mut comment_handler_get_comment_no_id_specified = HashMap::new();
    comment_handler_get_comment_no_id_specified.insert("en", "ID is not specified");
    comment_handler_get_comment_no_id_specified.insert("jp", "IDが指定されていません");
    map.insert(
        ErrorKey::CommentHandlerGetCommentNoIdSpecified,
        comment_handler_get_comment_no_id_specified,
    );

    let mut comment_handler_get_comment_no_task_id_specified = HashMap::new();
    comment_handler_get_comment_no_task_id_specified.insert("en", "Task ID is not specified");
    comment_handler_get_comment_no_task_id_specified.insert("jp", "タスクIDが指定されていません");
    map.insert(
        ErrorKey::CommentHandlerGetCommentNoTaskIdSpecified,
        comment_handler_get_comment_no_task_id_specified,
    );

    let mut comment_handler_get_comment_no_user_id_specified = HashMap::new();
    comment_handler_get_comment_no_user_id_specified.insert("en", "User ID is not specified");
    comment_handler_get_comment_no_user_id_specified.insert("jp", "ユーザーIDが指定されていません");
    map.insert(
        ErrorKey::CommentHandlerGetCommentNoUserIdSpecified,
        comment_handler_get_comment_no_user_id_specified,
    );

    let mut comment_handler_invalid_json_post = HashMap::new();
    comment_handler_invalid_json_post.insert(
        "en",
        "Invalid JSON post. JSON must be a valid comment object",
    );
    comment_handler_invalid_json_post.insert(
        "jp",
        "無効なJSONポストです。JSONは有効なコメントオブジェクトである必要があります",
    );
    map.insert(
        ErrorKey::CommentHandlerInvalidJsonPost,
        comment_handler_invalid_json_post,
    );

    let mut comment_handler_path_and_body_id_mismatch = HashMap::new();
    comment_handler_path_and_body_id_mismatch.insert("en", "Path and body ID mismatch");
    comment_handler_path_and_body_id_mismatch.insert("jp", "パスとボディのIDが一致しません");
    map.insert(
        ErrorKey::CommentHandlerPathAndBodyIdMismatch,
        comment_handler_path_and_body_id_mismatch,
    );

    let mut comment_handler_invalid_query = HashMap::new();
    comment_handler_invalid_query
        .insert("en", "Invalid query. Query must be a valid comment object");
    comment_handler_invalid_query.insert(
        "jp",
        "無効なクエリです。クエリは有効なコメントオブジェクトである必要があります",
    );
    map.insert(
        ErrorKey::CommentHandlerInvalidQuery,
        comment_handler_invalid_query,
    );

    let mut comment_handler_invalid_path = HashMap::new();
    comment_handler_invalid_path.insert("en", "Invalid path. Path must be a valid comment object");
    comment_handler_invalid_path.insert(
        "jp",
        "無効なパスです。パスは有効なコメントオブジェクトである必要があります",
    );
    map.insert(
        ErrorKey::CommentHandlerInvalidPath,
        comment_handler_invalid_path,
    );
}
