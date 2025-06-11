use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_user_assign_handler_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    // ユーザー割り当てハンドラ関連のエラーメッセージ
    let mut user_assign_handler_get_user_assigns_invalid_page = HashMap::new();
    user_assign_handler_get_user_assigns_invalid_page.insert(
        "en",
        "Invalid page or page size. Page must be greater than 0, and page size must be greater than 0 and less than 101",
    );
    user_assign_handler_get_user_assigns_invalid_page.insert(
        "jp", 
        "ページまたはページサイズが指定されていません。また、ページは1以上、ページサイズは1 <= 100である必要があります");
    map.insert(ErrorKey::UserAssignHandlerGetUserAssignsInvalidPage, user_assign_handler_get_user_assigns_invalid_page);

    let mut user_assign_handler_get_user_assigns_invalid_target = HashMap::new();
    user_assign_handler_get_user_assigns_invalid_target.insert(
        "en",
        "Invalid target. Target must be 'user' or 'task'",
    );
    user_assign_handler_get_user_assigns_invalid_target.insert(
        "jp", 
        "ターゲットが指定されていません。ターゲットは'user'または'task'である必要があります");
    map.insert(ErrorKey::UserAssignHandlerGetUserAssignsInvalidTarget, user_assign_handler_get_user_assigns_invalid_target);

    let mut user_assign_handler_get_user_assigns_no_id_specified = HashMap::new();
    user_assign_handler_get_user_assigns_no_id_specified.insert(
        "en",
        "ID is not specified",
    );
    user_assign_handler_get_user_assigns_no_id_specified.insert(
        "jp", 
        "IDが指定されていません");
    map.insert(ErrorKey::UserAssignHandlerGetUserAssignsNoIdSpecified, user_assign_handler_get_user_assigns_no_id_specified);

    let mut user_assign_handler_get_user_assigns_no_user_id_specified = HashMap::new();
    user_assign_handler_get_user_assigns_no_user_id_specified.insert(
        "en",
        "User ID is not specified",
    ); 
    user_assign_handler_get_user_assigns_no_user_id_specified.insert(
        "jp", 
        "ユーザーIDが指定されていません");
    map.insert(ErrorKey::UserAssignHandlerGetUserAssignsNoUserIdSpecified, user_assign_handler_get_user_assigns_no_user_id_specified);

    let mut user_assign_handler_get_user_assigns_no_task_id_specified = HashMap::new();
    user_assign_handler_get_user_assigns_no_task_id_specified.insert(
        "en",
        "Task ID is not specified",
    );
    user_assign_handler_get_user_assigns_no_task_id_specified.insert(
        "jp", 
        "タスクIDが指定されていません");
    map.insert(ErrorKey::UserAssignHandlerGetUserAssignsNoTaskIdSpecified, user_assign_handler_get_user_assigns_no_task_id_specified);

    let mut user_assign_handler_invalid_json_post = HashMap::new();
    user_assign_handler_invalid_json_post.insert(
        "en",
        "Invalid JSON in request body",
    );
    user_assign_handler_invalid_json_post.insert(
        "jp", 
        "リクエストボディに無効なJSONが指定されています");
    map.insert(ErrorKey::UserAssignHandlerInvalidJsonPost, user_assign_handler_invalid_json_post);

    let mut user_assign_handler_path_and_body_id_mismatch = HashMap::new();
    user_assign_handler_path_and_body_id_mismatch.insert(
        "en",
        "Path and body ID mismatch",
    );
    user_assign_handler_path_and_body_id_mismatch.insert(
        "jp", 
        "パスとボディのIDが一致しません");
    map.insert(ErrorKey::UserAssignHandlerPathAndBodyIdMismatch, user_assign_handler_path_and_body_id_mismatch);

    let mut user_assign_handler_invalid_query = HashMap::new();
    user_assign_handler_invalid_query.insert(
        "en",
        "Invalid query",
    );
    user_assign_handler_invalid_query.insert(
        "jp", 
        "無効なクエリです");
    map.insert(ErrorKey::UserAssignHandlerInvalidQuery, user_assign_handler_invalid_query);

    let mut user_assign_handler_invalid_path = HashMap::new();
    user_assign_handler_invalid_path.insert(
        "en",
        "Invalid path",
    );
    user_assign_handler_invalid_path.insert(
        "jp", 
        "無効なパスです");  
    map.insert(ErrorKey::UserAssignHandlerInvalidPath, user_assign_handler_invalid_path);   

}