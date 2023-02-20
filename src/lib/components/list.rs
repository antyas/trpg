use std::io::Stdout;

use crate::lib::{BaseComponent, Rect, Size, Style};

use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyEvent},
    style::{Attribute, PrintStyledContent, Stylize},
    QueueableCommand, Result,
};

const PREFIX_OFFSET: u16 = 2;

pub struct ListProps {
    pub items: Vec<String>,
}

#[derive(Default)]
pub struct List {
    pub index: usize,
    pub items: Vec<String>,
    style: Style,
    active_style: Style,
}

impl List {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    fn up(&mut self) {
        self.index = if self.index > 0 {
            self.index - 1
        } else {
            self.items.len() - 1
        }
    }

    fn down(&mut self) {
        self.index = if self.index + 1 < self.items.len() {
            self.index + 1
        } else {
            0
        }
    }

    pub fn size(&self) -> Size {
        (
            self.items
                .iter()
                .map(|i| i.chars().count())
                .max()
                .unwrap_or(0) as u16
                + PREFIX_OFFSET,
            self.items.len() as u16,
        )
    }
}

impl BaseComponent<ListProps, usize> for List {
    fn draw(&self, out: &mut Stdout, rect: &Rect) -> Result<()> {
        let (w, h) = rect.size();

        for (i, item) in self.items.iter().enumerate() {
            let (color, prefix) = if i == self.index {
                (self.active_style.fg, "> ")
            } else {
                (self.style.fg, "  ")
            };

            let styled = format!("{}{}", prefix, item)
                .with(color)
                .attribute(Attribute::Bold);

            out.queue(MoveTo(rect.x1, rect.y1 + i as u16))?;
            out.queue(PrintStyledContent(styled))?;
        }

        Ok(())
    }

    fn update(&mut self, props: ListProps) -> usize {
        self.items = props.items;

        if self.index >= self.items.len() {
            self.index = 0;
        }

        self.index
    }

    fn key(&mut self, key: KeyEvent) -> usize {
        if key.code == KeyCode::Up {
            self.up()
        };
        if key.code == KeyCode::Down {
            self.down()
        };

        self.index
    }
}
