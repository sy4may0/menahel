use crate::errors::messages::{get_error_message, ErrorKey};
use enum_iterator::{all, Sequence};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum TaskLevel {
    Major,
    Minor,
    Trivial,
}

impl TaskLevel {
    pub fn to_int(&self) -> i64 {
        match self {
            TaskLevel::Major => 0,
            TaskLevel::Minor => 1,
            TaskLevel::Trivial => 2,
        }
    }

    pub fn from_int(level: i64) -> Result<TaskLevel, anyhow::Error> {
        match level {
            0 => Ok(TaskLevel::Major),
            1 => Ok(TaskLevel::Minor),
            2 => Ok(TaskLevel::Trivial),
            _ => Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskLevelInvalid, format!("Level = {}", level)))),
        }
    }
}

pub fn get_max_task_level() -> i64 {
    all::<TaskLevel>().collect::<Vec<TaskLevel>>().len() as i64 - 1
}

pub enum TaskStatus {
    NotStarted,
    InProgress,
    Reviewing,
    Cancelled,
    Done,
} 

impl TaskStatus {
    pub fn to_int(&self) -> i64 {
        match self {
            TaskStatus::NotStarted => 0,
            TaskStatus::InProgress => 1,
            TaskStatus::Reviewing => 2,
            TaskStatus::Cancelled => 3,
            TaskStatus::Done => 4,
        }
    }

    pub fn from_int(status: i64) -> Result<TaskStatus, anyhow::Error> {
        match status {
            0 => Ok(TaskStatus::NotStarted),
            1 => Ok(TaskStatus::InProgress),
            2 => Ok(TaskStatus::Reviewing),
            3 => Ok(TaskStatus::Cancelled),
            4 => Ok(TaskStatus::Done),
            _ => Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskStatusInvalid, format!("Status = {}", status)))),
        }
    }

    pub fn to_short_string(&self) -> String {
        match self {
            TaskStatus::NotStarted => "ns".to_string(),
            TaskStatus::InProgress => "ip".to_string(),
            TaskStatus::Reviewing => "rv".to_string(),
            TaskStatus::Cancelled => "cn".to_string(),
            TaskStatus::Done => "dn".to_string(),
        }
    }

    pub fn from_short_string(status: &str) -> Result<TaskStatus, anyhow::Error> {
        match status {
            "ns" => Ok(TaskStatus::NotStarted),
            "ip" => Ok(TaskStatus::InProgress),
            "rv" => Ok(TaskStatus::Reviewing),
            "cn" => Ok(TaskStatus::Cancelled),
            "dn" => Ok(TaskStatus::Done),
            _ => Err(anyhow::anyhow!(get_error_message(ErrorKey::TaskStatusInvalid, format!("Status = {}", status)))),
        }
    }

}
