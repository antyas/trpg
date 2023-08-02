use crate::lib::{Component, Printer};

use crossterm::Result;

#[derive(Default)]
pub struct Text {
    rows: Vec<String>,
}

impl Text {
    pub fn new(rows: Vec<String>) -> Self {
        Self { rows }
    }

    pub fn height(&self) -> u16 {
        self.rows.len() as u16
    }

    // fn resize(&mut self, size: Size) {
    //     if self.size == size {
    //         return;
    //     }

    //     let (w, _) = size;
    //     let mut rows = Vec::<String>::new();
    //     let mut index = 0;

    //     for item in self.raw {
    //         if rows[index].len() < (w as usize + item.len()) {
    //             index += 1;
    //         }

    //         rows[index] = format!("{} {}", rows[index], item);
    //     }

    //     self.rows = rows;
    // }
}

impl Component for Text {
    fn draw(&self, printer: &Printer) -> Result<()> {
        self.rows
            .iter()
            .enumerate()
            .map(|(i, row)| printer.print(row, 0, i as u16));

        Ok(())
    }
}
