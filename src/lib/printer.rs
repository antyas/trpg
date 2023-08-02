use std::io::{Stdout, stdout};

use crossterm::{style::Print, cursor::MoveTo, Result, queue};

use super::{Style, Rect};

pub struct Printer {
    out: Stdout,
    pub style: Style,
    pub rect: Rect,
}

impl Printer {
    pub fn new(rect: Rect, style: Style) -> Self {
        Self {
            style,
            rect,
            out: stdout(),
        }
    }

    pub fn print(&self, text: &str, x: u16, y: u16) -> Result<()> {
        let x = self.rect.x1 + x;
        let y = self.rect.y1 + y;

        self.style.apply_colors()?;

        queue!(
            &self.out,
            MoveTo(x, y),
            Print(text),
        );

        self.style.reset_colors()?;

        Ok(())
    }

    pub fn hline(&self, x1: u16, x2: u16, y: u16) -> Result<()> {
        self.style.apply_colors()?;

        for x in x1 + 1..x2 {
            queue!(
                &self.out,
                MoveTo(x, y),
                Print('─'),
            );
        }

        self.style.reset_colors()?;
    
        Ok(())
    }

    pub fn border(&self, rect: Rect) -> Result<()> {
        self.style.apply_colors()?;

        queue!(
            &self.out,

            MoveTo(rect.x1, rect.y1),
            Print('┌'),

            MoveTo(rect.x2, rect.y1),
            Print('┐'),

            MoveTo(rect.x2, rect.y2),
            Print('┘'),

            MoveTo(rect.x1, rect.y2),
            Print('└'),
        );
    
        for x in rect.x1 + 1..rect.x2 {
            queue!(
                &self.out,
                MoveTo(x, rect.y1),
                Print('─'),
                MoveTo(x, rect.y2),
                Print('─'),
            );
        }
    
        for y in rect.y1 + 1..rect.y2 {
            queue!(
                &self.out,
                MoveTo(rect.x1, y),
                Print('│'),
                MoveTo(rect.x2, y),
                Print('│'),
            );
        }

        self.style.reset_colors()?;
    
        Ok(())
    }
}