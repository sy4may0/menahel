use std::collections::HashMap;

use crate::errors::messages::ErrorKey;

pub fn add_user_error_messages(
    map: &mut HashMap<ErrorKey, HashMap<&'static str, &'static str>>,
) {
    // ユーザー関連のエラーメッセージ
    let mut user_id_invalid = HashMap::new();
    user_id_invalid.insert("en", "User ID must be greater than 0");
    user_id_invalid.insert("jp", "ユーザーIDは0より大きくなければなりません");
    map.insert(ErrorKey::UserIdInvalid, user_id_invalid);

    let mut user_id_must_be_none = HashMap::new();
    user_id_must_be_none.insert("en", "User ID must be None. ID was specified in a process that cannot specify ID.");
    user_id_must_be_none.insert("jp", "ユーザーIDが指定できない処理でIDが指定されました。");
    map.insert(ErrorKey::UserIdMustBeNone, user_id_must_be_none);

    let mut user_name_empty = HashMap::new();
    user_name_empty.insert("en", "Username cannot be empty");
    user_name_empty.insert("jp", "ユーザー名は空にできません");
    map.insert(ErrorKey::UserNameEmpty, user_name_empty);

    let mut user_name_too_long = HashMap::new();
    user_name_too_long.insert("en", "Username cannot be longer than 128 characters");
    user_name_too_long.insert("jp", "ユーザー名は128文字以下である必要があります");
    map.insert(ErrorKey::UserNameTooLong, user_name_too_long);

    let mut user_name_invalid_chars = HashMap::new();
    user_name_invalid_chars.insert(
        "en",
        "Username can only contain alphanumeric characters, dots, and underscores",
    );
    user_name_invalid_chars.insert(
        "jp",
        "ユーザー名には英数字、ドット、アンダースコアのみ使用できます",
    );
    map.insert(
        ErrorKey::UserNameContainsInvalidCharacters,
        user_name_invalid_chars,
    );

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
    user_password_invalid.insert(
        "en",
        "Password must be a valid SHA-256 hash (lowercase hexadecimal)",
    );
    user_password_invalid.insert(
        "jp",
        "パスワードは有効なSHA-256ハッシュ（小文字の16進数）である必要があります",
    );
    map.insert(ErrorKey::UserPasswordInvalid, user_password_invalid);

    let mut user_create_failed = HashMap::new();
    user_create_failed.insert(
        "en",
        "Failed to create user due to database operation failure",
    );
    user_create_failed.insert("jp", "DB操作処理の問題によりユーザーの作成に失敗しました");
    map.insert(ErrorKey::UserCreateFailed, user_create_failed);

    let mut user_get_by_id_failed = HashMap::new();
    user_get_by_id_failed.insert(
        "en",
        "Failed to get user by ID due to database operation failure",
    );
    user_get_by_id_failed.insert(
        "jp",
        "DB操作処理の問題によりIDによるユーザーの取得に失敗しました",
    );
    map.insert(ErrorKey::UserGetByIdFailed, user_get_by_id_failed);

    let mut user_get_by_name_failed = HashMap::new();
    user_get_by_name_failed.insert(
        "en",
        "Failed to get user by name due to database operation failure",
    );
    user_get_by_name_failed.insert(
        "jp",
        "DB操作処理の問題により名前によるユーザーの取得に失敗しました",
    );
    map.insert(ErrorKey::UserGetByNameFailed, user_get_by_name_failed);

    let mut user_get_all_failed = HashMap::new();
    user_get_all_failed.insert(
        "en",
        "Failed to get all users due to database operation failure",
    );
    user_get_all_failed.insert(
        "jp",
        "DB操作処理の問題によりすべてのユーザーの取得に失敗しました",
    );
    map.insert(ErrorKey::UserGetAllFailed, user_get_all_failed);

    let mut user_update_failed = HashMap::new();
    user_update_failed.insert(
        "en",
        "Failed to update user due to database operation failure",
    );
    user_update_failed.insert("jp", "DB操作処理の問題によりユーザーの更新に失敗しました");
    map.insert(ErrorKey::UserUpdateFailed, user_update_failed);

    let mut user_delete_failed = HashMap::new();
    user_delete_failed.insert(
        "en",
        "Failed to delete user due to database operation failure",
    );
    user_delete_failed.insert("jp", "DB操作処理の問題によりユーザーの削除に失敗しました");
    map.insert(ErrorKey::UserDeleteFailed, user_delete_failed);

    let mut user_delete_failed_by_id_not_found = HashMap::new();
    user_delete_failed_by_id_not_found
        .insert("en", "Failed to delete user becouse user does not exist");
    user_delete_failed_by_id_not_found
        .insert("jp", "存在しないユーザーを削除しようとしました。");
    map.insert(
        ErrorKey::UserDeleteFailedByIdNotFound,
        user_delete_failed_by_id_not_found,
    );

    let mut user_get_users_count_failed = HashMap::new();
    user_get_users_count_failed.insert(
        "en",
        "Failed to get users count due to database operation failure",
    );
    user_get_users_count_failed.insert("jp", "DB操作処理の問題によりユーザーの数の取得に失敗しました");
    map.insert(ErrorKey::UserGetUsersCountFailed, user_get_users_count_failed);

    let mut user_get_users_pagenation_not_found = HashMap::new();
    user_get_users_pagenation_not_found.insert(
        "en",
        "No users found on the specified page",
    );
    user_get_users_pagenation_not_found.insert("jp", "指定されたページにユーザーが存在しません");
    map.insert(ErrorKey::UserGetUsersPagenationNotFound, user_get_users_pagenation_not_found);

}