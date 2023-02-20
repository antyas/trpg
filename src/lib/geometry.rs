use std::io::Stdout;

use crossterm::{cursor::MoveTo, style::Print, QueueableCommand, Result};

use super::Rect;

pub fn draw_hline(out: &mut Stdout, x1: u16, x2: u16, y: u16) -> Result<()> {
    for x in x1 + 1..x2 {
        out.queue(MoveTo(x, y))?;
        out.queue(Print('─'))?;
    }

    Ok(())
}

pub fn draw_border(out: &mut Stdout, rect: Rect) -> Result<()> {
    out.queue(MoveTo(rect.x1, rect.y1))?;
    out.queue(Print('┌'))?;

    out.queue(MoveTo(rect.x2, rect.y1))?;
    out.queue(Print('┐'))?;

    out.queue(MoveTo(rect.x2, rect.y2))?;
    out.queue(Print('┘'))?;

    out.queue(MoveTo(rect.x1, rect.y2))?;
    out.queue(Print('└'))?;

    for x in rect.x1 + 1..rect.x2 {
        out.queue(MoveTo(x, rect.y1))?;
        out.queue(Print('─'))?;
        out.queue(MoveTo(x, rect.y2))?;
        out.queue(Print('─'))?;
    }

    for y in rect.y1 + 1..rect.y2 {
        out.queue(MoveTo(rect.x1, y))?;
        out.queue(Print('│'))?;
        out.queue(MoveTo(rect.x2, y))?;
        out.queue(Print('│'))?;
    }

    Ok(())
}
