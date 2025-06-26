
pub enum Command {
    EmptyCommand,
    SetProject(String),
    InvalidCommand(String),
    Quit,
}

pub fn parse_command(command: &str) -> Command {
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    if parts.len() == 0 {
        return Command::EmptyCommand;
    }
    match parts[0] {
        "sp" => parse_set_project_cmd(command),
        "q" => Command::Quit,
        _ => Command::InvalidCommand(command.to_string()),
    }
}

fn parse_set_project_cmd(command: &str) -> Command {
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Command::InvalidCommand(command.to_string());
    }
    Command::SetProject(parts[1].to_string())
}