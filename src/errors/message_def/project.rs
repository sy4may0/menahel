use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_project_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
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

    let mut project_id_must_be_none = HashMap::new();
    project_id_must_be_none.insert("en", "ID was specified in a process that does not allow ID to be specified.");
    project_id_must_be_none.insert("jp", "プロジェクトIDを指定できない処理でIDが指定されました。");
    map.insert(ErrorKey::ProjectIdMustBeNone, project_id_must_be_none);

    let mut project_get_by_id_not_found = HashMap::new();
    project_get_by_id_not_found.insert("en", "Project ID not found");
    project_get_by_id_not_found.insert("jp", "プロジェクトIDが見つかりません");
    map.insert(ErrorKey::ProjectGetByIdNotFound, project_get_by_id_not_found);

    let mut project_get_by_name_not_found = HashMap::new();
    project_get_by_name_not_found.insert("en", "Project name not found");
    project_get_by_name_not_found.insert("jp", "プロジェクト名が見つかりません");
    map.insert(ErrorKey::ProjectGetByNameNotFound, project_get_by_name_not_found);

    let mut project_create_failed = HashMap::new();
    project_create_failed.insert(
        "en",
        "Failed to create project due to database operation failure",
    );
    project_create_failed.insert(
        "jp",
        "DB操作処理の問題によりプロジェクトの作成に失敗しました",
    );
    map.insert(ErrorKey::ProjectCreateFailed, project_create_failed);

    let mut project_get_by_id_failed = HashMap::new();
    project_get_by_id_failed.insert(
        "en",
        "Failed to get project by ID due to database operation failure",
    );
    project_get_by_id_failed.insert(
        "jp",
        "DB操作処理の問題によりIDによるプロジェクトの取得に失敗しました",
    );
    map.insert(ErrorKey::ProjectGetByIdFailed, project_get_by_id_failed);

    let mut project_get_by_name_failed = HashMap::new();
    project_get_by_name_failed.insert(
        "en",
        "Failed to get project by name due to database operation failure",
    );
    project_get_by_name_failed.insert(
        "jp",
        "DB操作処理の問題により名前によるプロジェクトの取得に失敗しました",
    );
    map.insert(ErrorKey::ProjectGetByNameFailed, project_get_by_name_failed);

    let mut project_get_all_failed = HashMap::new();
    project_get_all_failed.insert(
        "en",
        "Failed to get all projects due to database operation failure",
    );
    project_get_all_failed.insert(
        "jp",
        "DB操作処理の問題によりすべてのプロジェクトの取得に失敗しました",
    );
    map.insert(ErrorKey::ProjectGetAllFailed, project_get_all_failed);

    let mut project_update_failed = HashMap::new();
    project_update_failed.insert(
        "en",
        "Failed to update project due to database operation failure",
    );
    project_update_failed.insert(
        "jp",
        "DB操作処理の問題によりプロジェクトの更新に失敗しました",
    );
    map.insert(ErrorKey::ProjectUpdateFailed, project_update_failed);

    let mut project_delete_failed = HashMap::new();
    project_delete_failed.insert(
        "en",
        "Failed to delete project due to database operation failure",
    );
    project_delete_failed.insert(
        "jp",
        "DB操作処理の問題によりプロジェクトの削除に失敗しました",
    );
    map.insert(ErrorKey::ProjectDeleteFailed, project_delete_failed);

    let mut project_delete_failed_by_id_not_found = HashMap::new();
    project_delete_failed_by_id_not_found.insert(
        "en",
        "Failed to delete project becouse project does not exist",
    );
    project_delete_failed_by_id_not_found
        .insert("jp", "存在しないプロジェクトを削除しようとしました。");
    map.insert(
        ErrorKey::ProjectDeleteFailedByIdNotFound,
        project_delete_failed_by_id_not_found,
    );

    let mut project_get_projects_count_failed = HashMap::new();
    project_get_projects_count_failed.insert(
        "en",
        "Failed to get projects count due to database operation failure",
    );
    project_get_projects_count_failed.insert(
        "jp",
        "DB操作処理の問題によりプロジェクトの数の取得に失敗しました",
    );
    map.insert(ErrorKey::ProjectGetProjectsCountFailed, project_get_projects_count_failed);

    let mut project_get_pagenation_not_found = HashMap::new();
    project_get_pagenation_not_found.insert(
        "en",
        "No projects found on the specified page",
    );
    project_get_pagenation_not_found.insert(
        "jp",
        "指定ページにプロジェクトが存在しません",
    );
} 