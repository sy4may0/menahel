use color_eyre::{eyre::eyre, Result};

pub enum CommandKind {
    EmptyCommand,
    ChangeProject(String),
    CloseApp,
}

impl CommandKind {
    pub fn parse_command(command: &str) -> Result<Self> {
        let parts = command.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 0 {
            return Ok(CommandKind::EmptyCommand);
        }
        match parts[0] {
            "change-project" => parse_change_project(command),
            "cp" => parse_change_project(command),
            "q" => Ok(CommandKind::CloseApp),
            _ => Err(eyre!("Invalid command: {}", command)),
        }
    }
}

fn parse_change_project(command: &str) -> Result<CommandKind> {
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(eyre!("Invalid command: {}", command));
    }
    Ok(CommandKind::ChangeProject(parts[1].to_string()))
}