use std::error::Error;
use std::io::stdout;
use crossterm::{
    cursor::{EnableBlinking, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp}, 
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers}, 
    style::Print, 
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}, 
    ExecutableCommand,
    // ExecutableCommand,
};

use crate::buffer::buffer::Buffer;
use crate::command::Command;
pub struct Terminal;
impl Terminal {

    fn handle_keys(&self, event_code: KeyCode) -> Result<(), Box<dyn Error>>{
        // Basic Movement
        // j - down
        // k - up
        // h - left
        // l - right
        if let Some(command) = Command::from_key(event_code) {
            match command {
                Command::MoveUp => stdout().execute(MoveUp(1))?,
                Command::MoveDown => stdout().execute(MoveDown(1))?,
                Command::MoveLeft => stdout().execute(MoveLeft(1))?,
                Command::MoveRight => stdout().execute(MoveRight(1))?,
            };
        };
        Ok(())
    }

    pub fn print_events(&self, _buf: Buffer) -> Result<(), Box<dyn Error>> {
        loop{
            let event = read()?;
            match event {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    let mut curr_keys : String = "".to_string();
                    if event.modifiers != KeyModifiers::NONE {
                        let modifier : String = format!("{}+", event.modifiers);
                        curr_keys.extend([modifier.clone()]);
                    }
                    if curr_keys.len() >= 1 {
                        curr_keys.push_str(&format!(" {}", event.code));
                    } else {
                        curr_keys.push_str(&format!("{}", event.code));
                    } 
                    self.handle_keys(event.code.clone()).expect("Couldnt Do Command");
                    // stdout().execute(Print(curr_keys))?;
                    if event.code == KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    pub fn setup(&self, buf: Buffer, content : String) -> Result<(), Box<dyn Error>> {
        stdout().execute(Clear(ClearType::All))?;
        enable_raw_mode()?;
        stdout().execute(MoveTo(0,0))?;
        stdout().execute(EnableBlinking)?;
        stdout().execute(Print(format!("{}", content.clone())));
        if let Err(e) = self.print_events(buf) {
            return Err(e)
        }
        let _ = disable_raw_mode();
        Ok(())
    }
}



