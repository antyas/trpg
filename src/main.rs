use std::{
    io::Write,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};

mod lib;
mod state;
mod views;

use lib::{Component, Terminal, App, Printer, Style, Rect, AppEvent};
use state::State;
use views::MenuView;

pub struct GameApp {
    pub terminal: Terminal,
    pub root: Box<dyn Component>,
    pub key_event: KeyEvent,
    pub state: State,
}

impl Component for GameApp {
    fn draw(&self, printer: Printer) -> Result<()> {
        self.terminal.out.queue(Clear(ClearType::All))?;
        self.root.draw(printer, self)?;
        self.terminal.out.flush()?;

        Ok(())
    }

    fn on_key(&mut self, key: KeyEvent) -> AppEvent {
        if key.code == KeyCode::Esc {
            return AppEvent::Exit;
        }

        self.key = key;
        AppEvent::None
    }
}

fn main() -> Result<()> {
    let mut root: Box<dyn Component> = Box::new(MenuView::new());

    let mut app = GameApp {
        terminal: Terminal::new()?,
        root: Box::new(MenuView::new()),
        state: State::default(),
        key: KeyEvent { code: KeyCode::Null, modifiers: KeyModifiers::NONE },
    };

    let root_printer = Printer::new(Rect::full_terminal()?, Style::default());

    app.terminal.enable()?;

    loop {
        if poll(Duration::from_millis(100))? {
            let e = read()?;

            match e {
                Event::Key(key) => {
                    match app.on_key(key) {
                        AppEvent::Exit => break,
                        _ => {}   
                    }
                }
                Event::Resize(_, _) => {
                    app.terminal.on_resize();
                }
                Event::Mouse(_) => {}
            }
        };

        app.root.update(&mut app);
        app.key = KeyEvent { code: KeyCode::Null, modifiers: KeyModifiers::NONE };

        app.draw(root_printer)?;
    }

    app.terminal.disable()?;
    Ok(())
}
