use crossterm::event::KeyCode;

pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Command {
    pub fn from_key(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::Char('j') => Some(Command::MoveDown),
            KeyCode::Char('k') => Some(Command::MoveUp),
            KeyCode::Char('h') => Some(Command::MoveLeft),
            KeyCode::Char('l') => Some(Command::MoveRight),
            _ => None,
        }
    }
}
