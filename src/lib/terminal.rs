use std::io::{stdout, Write};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    ExecutableCommand,
    Result,
};

pub fn enable() -> Result<()> {
    let mut out = stdout();
    enable_raw_mode()?;
    out.execute(EnterAlternateScreen)?;
    out.execute(Hide)?;
    Ok(())
}

pub fn disable() -> Result<()> {
    let mut out = stdout();
    out.execute(Show)?;
    out.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn clear() -> Result<()> {
    let mut out = stdout();
    out.execute(Clear(ClearType::All))?;
    Ok(())
}

pub fn flush() -> Result<()> {
    let mut out = stdout();
    out.flush()?;
    Ok(())
}
