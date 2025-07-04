use crate::errors::messages::ErrorKey;
use std::collections::HashMap;

pub fn add_task_handler_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    let mut task_handler_get_tasks_invalid_page = HashMap::new();
    task_handler_get_tasks_invalid_page.insert("en", "Invalid page or page size. Page must be greater than 0, and page size must be less than 100.");
    task_handler_get_tasks_invalid_page.insert("jp", "ページまたはページサイズが指定されていません。また、ページは1以上、ページサイズは100以下である必要があります。");
    map.insert(
        ErrorKey::TaskHandlerGetTasksInvalidPage,
        task_handler_get_tasks_invalid_page,
    );

    let mut task_handler_get_tasks_invalid_target = HashMap::new();
    task_handler_get_tasks_invalid_target
        .insert("en", "Invalid target. Target must be 'all' or 'id'.");
    task_handler_get_tasks_invalid_target.insert(
        "jp",
        "ターゲットが無効です。ターゲットは'all'または'id'である必要があります",
    );
    map.insert(
        ErrorKey::TaskHandlerGetTasksInvalidTarget,
        task_handler_get_tasks_invalid_target,
    );

    let mut task_handler_get_tasks_no_id_specified = HashMap::new();
    task_handler_get_tasks_no_id_specified.insert("en", "No id specified");
    task_handler_get_tasks_no_id_specified.insert("jp", "IDが指定されていません");
    map.insert(
        ErrorKey::TaskHandlerGetTasksNoIdSpecified,
        task_handler_get_tasks_no_id_specified,
    );

    let mut task_handler_invalid_json_post = HashMap::new();
    task_handler_invalid_json_post.insert("en", "Invalid JSON format in request body");
    task_handler_invalid_json_post.insert("jp", "リクエストボディのJSON形式が無効です");
    map.insert(
        ErrorKey::TaskHandlerInvalidJsonPost,
        task_handler_invalid_json_post,
    );

    let mut task_handler_path_and_body_id_mismatch = HashMap::new();
    task_handler_path_and_body_id_mismatch.insert("en", "Path ID and body ID mismatch");
    task_handler_path_and_body_id_mismatch.insert("jp", "パスIDとボディIDが一致しません");
    map.insert(
        ErrorKey::TaskHandlerPathAndBodyIdMismatch,
        task_handler_path_and_body_id_mismatch,
    );

    let mut task_handler_invalid_query = HashMap::new();
    task_handler_invalid_query.insert("en", "Invalid query");
    task_handler_invalid_query.insert("jp", "クエリが無効です");
    map.insert(
        ErrorKey::TaskHandlerInvalidQuery,
        task_handler_invalid_query,
    );

    let mut task_handler_invalid_path = HashMap::new();
    task_handler_invalid_path.insert("en", "Invalid path");
    task_handler_invalid_path.insert("jp", "パスが無効です");
    map.insert(ErrorKey::TaskHandlerInvalidPath, task_handler_invalid_path);

    let mut task_handler_get_user_ids_parse_failed = HashMap::new();
    task_handler_get_user_ids_parse_failed.insert("en", "Failed to parse user IDs");
    task_handler_get_user_ids_parse_failed.insert("jp", "ユーザーIDフィルタのパースに失敗しました");
    map.insert(
        ErrorKey::TaskHandlerGetUserIdsParseFailed,
        task_handler_get_user_ids_parse_failed,
    );
}
