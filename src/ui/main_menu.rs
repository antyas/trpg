use std::io::Stdout;

use crate::{
    lib::{draw_border, Component, Indents, Rect},
    state::State,
};

use super::{PRIMARY_COLOR, SECONDARY_COLOR};

use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyEvent},
    style::{Attribute, PrintStyledContent, Stylize},
    QueueableCommand, Result,
};

const PREFIX_OFFSET: u16 = 2;

const MENU_ITEMS: [&'static str; 3] = ["Загрузить", "Новая игра", "Выход"];

#[derive(Default)]
pub struct MainMenu {
    index: u8,
}

impl MainMenu {
    fn up(&mut self) {
        self.index = if self.index > 0 {
            self.index - 1
        } else {
            (MENU_ITEMS.len() - 1) as u8
        }
    }

    fn down(&mut self) {
        self.index = if self.index + 1 < MENU_ITEMS.len() as u8 {
            self.index + 1
        } else {
            0
        }
    }
}

impl Component<State> for MainMenu {
    fn draw(&self, out: &mut Stdout, rect: &Rect, _: &State) -> Result<()> {
        let (w, h) = rect.size();

        let content_w = MENU_ITEMS
            .iter()
            .map(|i| i.chars().count())
            .max()
            .unwrap_or(0) as u16
            + PREFIX_OFFSET;
        let content_h = MENU_ITEMS.len() as u16;

        let content_rect = Rect::new(
            w / 2 - content_w / 2,
            h / 2 - content_h / 2,
            w / 2 + content_w / 2,
            h / 2 + content_h / 2,
        );

        let border_rect = content_rect.with_margin(Indents::new(2, 3, 2, 2));

        draw_border(out, border_rect)?;

        for (i, item) in MENU_ITEMS.iter().enumerate() {
            let (color, prefix) = if i as u8 == self.index {
                (PRIMARY_COLOR, "> ")
            } else {
                (SECONDARY_COLOR, "  ")
            };

            let styled = format!("{}{}", prefix, item)
                .with(color)
                .attribute(Attribute::Bold);

            out.queue(MoveTo(content_rect.x1, content_rect.y1 + i as u16))?;
            out.queue(PrintStyledContent(styled))?;
        }

        Ok(())
    }

    fn key(&mut self, key: KeyEvent, _: &mut State) {
        if key.code == KeyCode::Up {
            self.up()
        }
        if key.code == KeyCode::Down {
            self.down()
        }
    }
}
