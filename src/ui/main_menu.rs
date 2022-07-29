use crate::{app::App};

use super::{Component, TEXT_FOCUS_COLOR, TEXT_PRIMARY_COLOR};
use crossterm::{
    Result, 
    terminal::size, 
    queue, 
    cursor::MoveTo, 
    style::{Stylize, Attribute, PrintStyledContent, Print},
    event::{Event, KeyCode}
};

const WIDTH: u16 = 30;

const MENU_ITEMS: [&'static str; 2] = [
    "Новая игра",
    "Выход"
];

#[derive(Default)]
pub struct MainMenu {
    index: u8,
}

impl MainMenu {
    fn up(&mut self) {
        self.index = if self.index + 1 < MENU_ITEMS.len() as u8 {
            self.index + 1
        } else {
            0
        }
    }

    fn down(&mut self) {
        self.index = if self.index > 1 {
            self.index - 1
        } else {
            (MENU_ITEMS.len() - 1) as u8
        }
    }
}

impl Component for MainMenu {
    fn draw(&self, app: &mut App) -> Result<()> {
        let (w, h) = size()?;

        let x = (w - WIDTH) / 2;
        let y = (h - MENU_ITEMS.len() as u16) / 2;
        let mut line = 0;

        for item in MENU_ITEMS {
            let offset: u16 = (WIDTH - item.chars().count() as u16) / 2;
            let color = if line == self.index { TEXT_FOCUS_COLOR } else { TEXT_PRIMARY_COLOR };

            let styled = (item)
                .with(color)
                .attribute(Attribute::Bold);

            queue!(
                app.out,
                MoveTo(x + offset, y + line as u16),
                PrintStyledContent(styled),
            )?;

            line += 1;
        }

        Ok(())
    }

    fn on_state_update(&mut self, app: &mut App) {
        todo!()
    }

    fn on_terminal_event(&mut self, app: &mut App, event: Event) {
        match event {
            Event::Key(e) => {
                if e.code == KeyCode::Up { self.up() }
                if e.code == KeyCode::Down { self.down() }
            },
            _ => (),
        }
    }
}