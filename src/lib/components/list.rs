use crate::lib::{Component, Size, Style, printer};

use crossterm::{
    event::{KeyCode, KeyEvent},
    Result,
};
use printer::Printer;

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

impl Component for List {
    fn draw(&self, printer: &Printer) -> Result<()> {
        for (i, item) in self.items.iter().enumerate() {
            let prefix = if i == self.index { "> " } else { "  " };

            let string = format!("{}{}", prefix, item);

            printer.print(&string, 0, i as u16)?;
        }

        Ok(())
    }

    fn on_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Up {
            self.up()
        };
        if key.code == KeyCode::Down {
            self.down()
        };
    }
}
