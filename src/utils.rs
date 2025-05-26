use regex::Regex;
use email_address::EmailAddress;

pub fn validate_user_name(name: &str) -> Result<(), anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Username cannot be empty"));
    }

    if name.len() > 128 {
        return Err(anyhow::anyhow!("Username cannot be longer than 128 characters"));
    }

    let re = Regex::new(r"^[a-zA-Z0-9._]+$").unwrap();
    if !re.is_match(name) {
        return Err(anyhow::anyhow!("Username can only contain alphanumeric characters, dots, and underscores"));
    }

    Ok(())
}

pub fn validate_user_email(email: &str) -> Result<(), anyhow::Error> {
    if email.is_empty() {
        return Err(anyhow::anyhow!("Email cannot be empty"));
    }

    if email.len() > 254 {
        return Err(anyhow::anyhow!("Email cannot be longer than 254 characters"));
    }

    if !EmailAddress::is_valid(email) {
        return Err(anyhow::anyhow!("Invalid email address"));
    } 

    Ok(())
}

pub fn validate_user_password(password: &str) -> Result<(), anyhow::Error> {
    if password.is_empty() {
        return Err(anyhow::anyhow!("Password cannot be empty"));
    }

    let re_sha256 = Regex::new(r"^[a-f0-9]{64}$").unwrap();
    if !re_sha256.is_match(password) {
        return Err(anyhow::anyhow!("Password must be a valid SHA-256 hash (lowercase hexadecimal)"));
    }

    Ok(())
}

pub fn validate_project_name(name: &str) -> Result<(), anyhow::Error> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Project name cannot be empty"));
    }

    if name.len() > 128 {
        return Err(anyhow::anyhow!("Project name cannot be longer than 128 characters"));
    }

    Ok(())
}