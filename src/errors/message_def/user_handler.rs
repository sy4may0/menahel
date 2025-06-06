use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_user_handler_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    // ユーザーハンドラ関連のエラーメッセージ
    let mut user_handler_get_users_invalid_page = HashMap::new();
    user_handler_get_users_invalid_page.insert(
        "en",
        "Invalid page or page size. Page must be greater than 0, and page size must be greater than 0 and less than 101",
    );
    user_handler_get_users_invalid_page.insert(
        "jp", 
        "ページまたはページサイズが指定されていません。また、ページは1以上、ページサイズは1 <= 100である必要があります");
    map.insert(
        ErrorKey::UserHandlerGetUsersInvalidPage,
        user_handler_get_users_invalid_page,
    );

    let mut user_get_by_id_not_found = HashMap::new();
    user_get_by_id_not_found.insert("en", "User not found");
    user_get_by_id_not_found.insert("jp", "ユーザーが見つかりません");
    map.insert(ErrorKey::UserGetByIdNotFound, user_get_by_id_not_found);

    let mut user_get_by_name_not_found = HashMap::new();
    user_get_by_name_not_found.insert("en", "User not found");
    user_get_by_name_not_found.insert("jp", "ユーザーが見つかりません");
    map.insert(ErrorKey::UserGetByNameNotFound, user_get_by_name_not_found);

    let mut user_handler_get_users_invalid_target = HashMap::new();
    user_handler_get_users_invalid_target.insert("en", "Invalid target. Target must be 'all', 'name', or 'id'");
    user_handler_get_users_invalid_target.insert("jp", "ターゲットが無効です。ターゲットは'all'、'name'、または'id'である必要があります");
    map.insert(ErrorKey::UserHandlerGetUsersInvalidTarget, user_handler_get_users_invalid_target);

    let mut user_handler_get_users_no_name_specified = HashMap::new();
    user_handler_get_users_no_name_specified.insert("en", "Name is not specified");
    user_handler_get_users_no_name_specified.insert("jp", "名前が指定されていません");
    map.insert(ErrorKey::UserHandlerGetUsersNoNameSpecified, user_handler_get_users_no_name_specified);

    let mut user_handler_get_users_no_id_specified = HashMap::new();
    user_handler_get_users_no_id_specified.insert("en", "ID is not specified");
    user_handler_get_users_no_id_specified.insert("jp", "IDが指定されていません");
    map.insert(ErrorKey::UserHandlerGetUsersNoIdSpecified, user_handler_get_users_no_id_specified);

    let mut user_handler_path_and_body_id_mismatch = HashMap::new();
    user_handler_path_and_body_id_mismatch.insert("en", "Path ID and body ID mismatch");
    user_handler_path_and_body_id_mismatch.insert("jp", "パスIDとボディIDが一致しません");
    map.insert(ErrorKey::UserHandlerPathAndBodyIdMismatch, user_handler_path_and_body_id_mismatch);

    let mut user_handler_invalid_json_post = HashMap::new();
    user_handler_invalid_json_post.insert("en", "Invalid JSON format");
    user_handler_invalid_json_post.insert("jp", "JSON形式が無効です");
    map.insert(ErrorKey::UserHandlerInvalidJsonPost, user_handler_invalid_json_post);

}