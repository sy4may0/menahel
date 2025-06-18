use crate::errors::messages::ErrorKey;
use std::collections::HashMap;

pub fn add_project_handler_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    let mut project_handler_get_projects_invalid_page = HashMap::new();
    project_handler_get_projects_invalid_page.insert("en", "Invalid page or page size. Page must be greater than 0, and page size must be less than 100.");
    project_handler_get_projects_invalid_page.insert("jp", "ページまたはページサイズが指定されていません。また、ページは1以上、ページサイズは100以下である必要があります。");
    map.insert(
        ErrorKey::ProjectHandlerGetProjectsInvalidPage,
        project_handler_get_projects_invalid_page,
    );

    let mut project_handler_get_projects_invalid_target = HashMap::new();
    project_handler_get_projects_invalid_target.insert(
        "en",
        "Invalid target. Target must be 'all', 'name', or 'id'",
    );
    project_handler_get_projects_invalid_target.insert(
        "jp",
        "ターゲットが無効です。ターゲットは'all'、'name'、または'id'である必要があります",
    );
    map.insert(
        ErrorKey::ProjectHandlerGetProjectsInvalidTarget,
        project_handler_get_projects_invalid_target,
    );

    let mut project_handler_get_projects_no_name_specified = HashMap::new();
    project_handler_get_projects_no_name_specified.insert("en", "No name specified");
    project_handler_get_projects_no_name_specified.insert("jp", "名前が指定されていません");
    map.insert(
        ErrorKey::ProjectHandlerGetProjectsNoNameSpecified,
        project_handler_get_projects_no_name_specified,
    );

    let mut project_handler_get_projects_no_id_specified = HashMap::new();
    project_handler_get_projects_no_id_specified.insert("en", "No id specified");
    project_handler_get_projects_no_id_specified.insert("jp", "IDが指定されていません");
    map.insert(
        ErrorKey::ProjectHandlerGetProjectsNoIdSpecified,
        project_handler_get_projects_no_id_specified,
    );

    let mut project_handler_path_and_body_id_mismatch = HashMap::new();
    project_handler_path_and_body_id_mismatch.insert("en", "Path and body id mismatch");
    project_handler_path_and_body_id_mismatch.insert("jp", "パスとボディのIDが一致しません");
    map.insert(
        ErrorKey::ProjectHandlerPathAndBodyIdMismatch,
        project_handler_path_and_body_id_mismatch,
    );

    let mut project_handler_invalid_json_post = HashMap::new();
    project_handler_invalid_json_post.insert("en", "Invalid JSON format");
    project_handler_invalid_json_post.insert("jp", "JSON形式が無効です");
    map.insert(
        ErrorKey::ProjectHandlerInvalidJsonPost,
        project_handler_invalid_json_post,
    );

    let mut project_handler_invalid_query = HashMap::new();
    project_handler_invalid_query.insert("en", "Invalid query");
    project_handler_invalid_query.insert("jp", "クエリが無効です");
    map.insert(
        ErrorKey::ProjectHandlerInvalidQuery,
        project_handler_invalid_query,
    );

    let mut project_handler_invalid_path = HashMap::new();
    project_handler_invalid_path.insert("en", "Invalid path");
    project_handler_invalid_path.insert("jp", "パスが無効です");
    map.insert(
        ErrorKey::ProjectHandlerInvalidPath,
        project_handler_invalid_path,
    );
}
