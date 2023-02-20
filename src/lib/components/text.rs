use std::io::Stdout;

use super::super::{BaseComponent, Rect, Style};

use crossterm::{cursor::MoveTo, style::Print, QueueableCommand, Result};

pub struct TextProps {
    rows: Vec<String>,
}

#[derive(Default)]
pub struct Text {
    rows: Vec<String>,
    style: Style,
}

impl Text {
    pub fn new(rows: Vec<String>) -> Self {
        Self {
            rows,
            ..Self::default()
        }
    }

    pub fn height(&self) -> u16 {
        self.rows.len() as u16
    }

    pub fn set_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl BaseComponent<TextProps, ()> for Text {
    fn resize(&mut self, rect: Rect, data: TextProps) {
        let (w, _) = rect.size();
        let mut rows = Vec::<String>::new();
        let mut index = 0;

        for item in data.rows {
            if rows[index].len() < (w as usize + item.len()) {
                index += 1;
            }

            rows[index] = format!("{} {}", rows[index], item);
        }

        self.rows = rows;
    }

    fn draw(&self, out: &mut Stdout, rect: &Rect) -> Result<()> {
        self.style.apply_colors(out)?;

        let (_, h) = rect.size();
        let start = h - self.rows.len() as u16;

        for (i, row) in self.rows.iter().enumerate() {
            out.queue(MoveTo(0, start + i as u16))?;
            out.queue(Print(row))?;
        }

        self.style.reset_colors(out)?;

        Ok(())
    }
}
