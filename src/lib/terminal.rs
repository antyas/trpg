use std::io::Stdout;

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, Result,
};

pub struct Terminal {
    pub out: Stdout,
}

impl Terminal {
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
}
