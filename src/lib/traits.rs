use crossterm::{Result, event::KeyEvent};

pub enum AppEvent {
    None,
    Exit,
} 

use super::Printer;

pub trait Component {
    fn draw(&self, printer: Printer) -> Result<()> {
        Ok(())
    }

    fn on_key(&mut self, event: KeyEvent) {}
}
