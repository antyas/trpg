use std::io::{stdout, Stdout};
use crate::{ui::Component, state::State};
use crossterm::{
    ExecutableCommand, Result,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, self}, 
    event::Event, cursor::{Hide, Show},
};

pub struct App {
    pub out: Stdout,
    pub state: State,
}

impl App {
    pub fn new() -> Self {
        Self { 
            out: stdout(),
            state: State::new(),
        }
    }

    pub fn enable_terminal(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.out.execute(EnterAlternateScreen)?;
        self.out.execute(Hide)?;
        Ok(())
    }

    pub fn disable_terminal(&mut self) -> Result<()> {
        self.out.execute(Show)?;
        self.out.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}