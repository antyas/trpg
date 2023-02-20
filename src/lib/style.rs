use std::io::Stdout;

use crossterm::{
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    QueueableCommand, Result,
};

pub struct Style {
    pub fg: Color,
    pub bg: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::Green,
            bg: Color::Black,
        }
    }
}

impl Style {
    pub fn apply_colors(&self, out: &mut Stdout) -> Result<()> {
        out.queue(SetForegroundColor(self.fg))?;
        out.queue(SetBackgroundColor(self.bg))?;
        Ok(())
    }

    pub fn reset_colors(&self, out: &mut Stdout) -> Result<()> {
        out.queue(ResetColor)?;
        Ok(())
    }
}
