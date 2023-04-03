use std::io::{Stdout, stdout};

use crossterm::{
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor, Attribute, SetAttribute},
    Result,
    queue,
};

pub struct Style {
    out: Stdout,
    pub fg: Color,
    pub bg: Color,
    pub attribute: Attribute,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            out: stdout(),
            fg: Color::Green,
            bg: Color::Black,
            attribute: Attribute::Reset,
        }
    }
}

impl Style {
    pub fn apply_colors(&self) -> Result<()> {
        queue!(
            self.out,
            SetForegroundColor(self.fg),
            SetBackgroundColor(self.bg),
            SetAttribute(self.attribute),
        )
    }

    pub fn reset_colors(&self) -> Result<()> {
        queue!(self.out, ResetColor, SetAttribute(Attribute::Reset))
    }
}
