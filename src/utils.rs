use regex::Regex;
use email_address::EmailAddress;
use crate::enums::{TaskStatus, TaskLevel};
use crate::errors::messages::{get_error_message, ErrorKey};

pub fn validate_user_id(id: Option<i64>) -> Result<(), anyhow::Error> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() > 0 {
        return Ok(());
    } else {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserIdInvalid, format!("ID = {}", id.unwrap()))));
    }
}

pub fn validate_user_name(name: &str) -> Result<(), anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserNameEmpty, format!("Name = {}", name))));
    }

    if name.len() > 128 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserNameTooLong, format!("Name = {}", name))));
    }

    let re = Regex::new(r"^[a-zA-Z0-9._]+$").unwrap();
    if !re.is_match(name) {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserNameContainsInvalidCharacters, format!("Name = {}", name))));
    }

    Ok(())
}

pub fn validate_user_email(email: &str) -> Result<(), anyhow::Error> {
    if email.is_empty() {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserEmailEmpty, format!("Email = {}", email))));
    }

    if email.len() > 254 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserEmailTooLong, format!("Email = {}", email))));
    }

    if !EmailAddress::is_valid(email) {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserEmailInvalid, format!("Email = {}", email))));
    } 

    Ok(())
}

pub fn validate_user_password(password: &str) -> Result<(), anyhow::Error> {
    if password.is_empty() {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserPasswordEmpty, format!("Password = {}", password))));
    }

    let re_sha256 = Regex::new(r"^[a-f0-9]{64}$").unwrap();
    if !re_sha256.is_match(password) {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserPasswordInvalid, format!("Password = {}", password))));
    }

    Ok(())
}

pub fn validate_project_id(id: Option<i64>) -> Result<(), anyhow::Error> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() > 0 {
        return Ok(());
    } else {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::ProjectIdInvalid, format!("ID = {}", id.unwrap()))));
    }
}

pub fn validate_project_name(name: &str) -> Result<(), anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::ProjectNameEmpty, format!("Name = {}", name))));
    }

    if name.len() > 128 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::ProjectNameTooLong, format!("Name = {}", name))));
    }

    Ok(())
}



pub fn validate_task_id(id: Option<i64>) -> Result<(), anyhow::Error> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() > 0 {
        return Ok(());
    } else {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskIdInvalid, format!("ID = {}", id.unwrap()))));
    }
}

pub fn validate_task_project_id(id: i64) -> Result<(), anyhow::Error> {
    if id <= 0 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskProjectIdInvalid, format!("ID = {}", id))));
    }

    Ok(())
}

pub fn validate_task_parent_id(id: Option<i64>) -> Result<(), anyhow::Error> {
    if id.is_none() {
        return Ok(());
    } else if id.is_some() && id.unwrap() > 0 {
        return Ok(());
    } else {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskParentIdInvalid, format!("ID = {}", id.unwrap()))));
    }
}

pub fn validate_task_level(level: i64) -> Result<(), anyhow::Error> {
    TaskLevel::from_int(level).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}

pub fn validate_task_status(status: i64) -> Result<(), anyhow::Error> {
    TaskStatus::from_int(status).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(())
}

pub fn validate_task_name(name: &str) -> Result<(), anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskNameEmpty, format!("Name = {}", name))));
    }

    if name.len() > 128 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskNameTooLong, format!("Name = {}", name))));
    }

    Ok(())
}

pub fn validate_task_description(description: Option<&String>) -> Result<(), anyhow::Error> {
    if description.is_none() {
        return Ok(());
    }

    if description.unwrap().len() > 1024 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskDescriptionTooLong, format!("Description = {}", description.unwrap()))));
    }

    Ok(())
}

pub fn validate_task_unix_timestamp(timestamp: i64) -> Result<(), anyhow::Error> {
    if timestamp < 0 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskTimestampInvalid, format!("Timestamp = {}", timestamp))));
    }

    Ok(())
}

pub fn validate_task_unix_timestamp_or_none(timestamp: Option<i64>) -> Result<(), anyhow::Error> {
    if timestamp.is_none() {
        return Ok(());
    } else if timestamp.is_some() && timestamp.unwrap() > 0 {
        return Ok(());
    } else {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskTimestampInvalid, format!("Timestamp = {}", timestamp.unwrap()))));
    }
}

pub fn validate_user_assign_user_id(id: i64) -> Result<(), anyhow::Error> {
    if id <= 0 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignUserIdInvalid, format!("ID = {}", id))));
    }

    Ok(())
}

pub fn validate_user_assign_task_id(id: i64) -> Result<(), anyhow::Error> {
    if id <= 0 {
        return Err(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignTaskIdInvalid, format!("ID = {}", id))));
    }

    Ok(())
}