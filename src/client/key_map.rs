use ratatui::crossterm::event::KeyCode;
pub enum UiKeyMap {
    FocusToCommandPane,
    ChangeFocusToLeft,
    ChangeFocusToRight,
    FocusBackFromCommandPane,
    UnDefined
}

impl UiKeyMap {
    pub fn key(key: KeyCode) -> Self {
        match key {
            KeyCode::Char(':') => UiKeyMap::FocusToCommandPane,
            KeyCode::Char('h') => UiKeyMap::ChangeFocusToLeft,
            KeyCode::Char('l') => UiKeyMap::ChangeFocusToRight,
            KeyCode::Esc => UiKeyMap::FocusBackFromCommandPane,
            _ => UiKeyMap::UnDefined,
        }
    }
}