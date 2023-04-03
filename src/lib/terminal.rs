use std::io::{Stdout, stdout};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{size, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, Result,
};

use super::Size;

pub struct Terminal {
    pub out: Stdout,
    pub size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        Ok(Self {
            out: stdout(),
            size: size()?,
        })
    }

    pub fn enable(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.out.execute(EnterAlternateScreen)?;
        self.out.execute(Hide)?;
        Ok(())
    }

    pub fn disable(&mut self) -> Result<()> {
        self.out.execute(Show)?;
        self.out.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn on_resize(&mut self) -> Result<()> {
        Ok(self.size = size()?)
    }
}
