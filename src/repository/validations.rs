use crate::enums::{TaskLevel, TaskStatus};
use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use email_address::EmailAddress;
use regex::Regex;

pub fn validate_user_id(id: Option<i64>) -> Result<(), DBAccessError> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() >= 0 {
        return Ok(());
    } else {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserIdInvalid,
            format!("ID = {}", id.unwrap()),
        )));
    }
}

pub fn validate_user_id_is_none(id: Option<i64>) -> Result<(), DBAccessError> {
    if id.is_none() {
        return Ok(());
    } else {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserIdMustBeNone,
            format!("ID = {}", id.unwrap()),
        )));
    }
}

pub fn validate_user_name(name: &str) -> Result<(), DBAccessError> {
    if name.is_empty() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserNameEmpty,
            format!("Name = {}", name),
        )));
    }

    if name.len() > 128 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserNameTooLong,
            format!("Name = {}", name),
        )));
    }

    let re = Regex::new(r"^[a-zA-Z0-9._]+$").unwrap();
    if !re.is_match(name) {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserNameContainsInvalidCharacters,
            format!("Name = {}", name),
        )));
    }

    Ok(())
}

pub fn validate_user_email(email: &str) -> Result<(), DBAccessError> {
    if email.is_empty() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserEmailEmpty,
            format!("Email = {}", email),
        )));
    }

    if email.len() > 254 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserEmailTooLong,
            format!("Email = {}", email),
        )));
    }

    if !EmailAddress::is_valid(email) {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserEmailInvalid,
            format!("Email = {}", email),
        )));
    }

    Ok(())
}

pub fn validate_user_password(password: &str) -> Result<(), DBAccessError> {
    if password.is_empty() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserPasswordEmpty,
            format!("Password = {}", password),
        )));
    }

    let re_sha256 = Regex::new(r"^[a-f0-9]{64}$").unwrap();
    if !re_sha256.is_match(password) {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserPasswordInvalid,
            format!("Password = {}", password),
        )));
    }

    Ok(())
}

pub fn validate_project_id(id: Option<i64>) -> Result<(), DBAccessError> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() >= 0 {
        return Ok(());
    } else {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::ProjectIdInvalid,
            format!("ID = {}", id.unwrap()),
        )));
    }
}

pub fn validate_project_name(name: &str) -> Result<(), DBAccessError> {
    if name.is_empty() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::ProjectNameEmpty,
            format!("Name = {}", name),
        )));
    }

    if name.len() > 128 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::ProjectNameTooLong,
            format!("Name = {}", name),
        )));
    }

    Ok(())
}

pub fn validate_task_id(id: Option<i64>) -> Result<(), DBAccessError> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() >= 0 {
        return Ok(());
    } else {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskIdInvalid,
            format!("ID = {}", id.unwrap()),
        )));
    }
}

pub fn validate_task_project_id(id: i64) -> Result<(), DBAccessError> {
    if id < 0 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskProjectIdInvalid,
            format!("ID = {}", id),
        )));
    }

    Ok(())
}

pub fn validate_task_parent_id(id: Option<i64>) -> Result<(), DBAccessError> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() >= 0 {
        return Ok(());
    } else {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskParentIdInvalid,
            format!("ID = {}", id.unwrap()),
        )));
    }
}

pub fn validate_task_level(level: i64) -> Result<(), DBAccessError> {
    TaskLevel::from_int(level).map_err(|e| {
        DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskLevelInvalid,
            format!("{}", e.to_string()),
        ))
    })?;
    Ok(())
}

pub fn validate_task_status(status: i64) -> Result<(), DBAccessError> {
    TaskStatus::from_int(status).map_err(|e| {
        DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskStatusInvalid,
            format!("{}", e.to_string()),
        ))
    })?;
    Ok(())
}

pub fn validate_task_name(name: &str) -> Result<(), DBAccessError> {
    if name.is_empty() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskNameEmpty,
            format!("Name = {}", name),
        )));
    }

    if name.len() > 128 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskNameTooLong,
            format!("Name = {}", name),
        )));
    }

    Ok(())
}

pub fn validate_task_description(description: Option<&String>) -> Result<(), DBAccessError> {
    if description.is_none() {
        return Ok(());
    }

    if description.unwrap().len() > 1024 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskDescriptionTooLong,
            format!("Description = {}", description.unwrap()),
        )));
    }

    Ok(())
}

pub fn validate_task_unix_timestamp(timestamp: i64) -> Result<(), DBAccessError> {
    if timestamp < 0 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskTimestampInvalid,
            format!("Timestamp = {}", timestamp),
        )));
    }

    Ok(())
}

pub fn validate_task_unix_timestamp_or_none(timestamp: Option<i64>) -> Result<(), DBAccessError> {
    if timestamp.is_none() {
        return Ok(());
    } else if timestamp.is_some() && timestamp.unwrap() > 0 {
        return Ok(());
    } else {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::TaskTimestampInvalid,
            format!("Timestamp = {}", timestamp.unwrap()),
        )));
    }
}

pub fn validate_user_assign_id(id: Option<i64>) -> Result<(), DBAccessError> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() >= 0 {
        return Ok(());
    } else {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserAssignIdInvalid,
            format!("ID = {}", id.unwrap()),
        )));
    }
}

pub fn validate_user_assign_user_id(id: i64) -> Result<(), DBAccessError> {
    if id < 0 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserAssignUserIdInvalid,
            format!("ID = {}", id),
        )));
    }

    Ok(())
}

pub fn validate_user_assign_task_id(id: i64) -> Result<(), DBAccessError> {
    if id < 0 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::UserAssignTaskIdInvalid,
            format!("ID = {}", id),
        )));
    }

    Ok(())
}

pub fn validate_comment_user_id(id: i64) -> Result<(), DBAccessError> {
    if id < 0 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::CommentUserIdInvalid,
            format!("ID = {}", id),
        )));
    }

    Ok(())
}

pub fn validate_comment_task_id(id: i64) -> Result<(), DBAccessError> {
    if id < 0 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::CommentTaskIdInvalid,
            format!("ID = {}", id),
        )));
    }

    Ok(())
}

pub fn validate_comment_content(content: &str) -> Result<(), DBAccessError> {
    if content.is_empty() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::CommentContentEmpty,
            format!("Content = {}", content),
        )));
    }

    if content.len() > 2024 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::CommentContentTooLong,
            format!("Content = {}", content),
        )));
    }

    Ok(())
}

pub fn validate_pagination(page: Option<&i32>, page_size: Option<&i32>, max_count: &i64) -> Result<(), DBAccessError> {
    if page.is_none() && page_size.is_some() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::NoPageSpecified,
            format!("Page = {:?}, PageSize = {:?}", page, page_size),
        )));
    }

    if page.is_some() && page_size.is_none() {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::NoPageSizeSpecified,
            format!("Page = {:?}, PageSize = {:?}", page, page_size),
        )));
    }

    if page.is_none() && page_size.is_none() {
        return Ok(());
    }

    let page = page.as_ref().unwrap();
    let page_size = page_size.as_ref().unwrap();

    if **page < 1 || **page_size < 1 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::InvalidPagination,
            format!("Page = {}, PageSize = {}", *page, *page_size),
        )));
    }

    if **page_size > 100 {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::PageSizeTooLarge,
            format!("Page = {}, PageSize = {}", *page, *page_size),
        )));
    }

    let offset = (*page - 1) * *page_size;
    if offset as i64 > *max_count {
        return Err(DBAccessError::ValidationError(get_error_message(
            ErrorKey::InvalidPagination,
            format!("Offset = {}, MaxCount = {}", offset, *max_count),
        )));
    }

    Ok(())
}
