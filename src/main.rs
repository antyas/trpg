use std::time::Duration;

use crossterm::{
    event::{poll, read, Event, KeyCode},
    Result,
};

mod lib;
mod views;

use lib::{Component, terminal, Printer, Style, Rect};
use views::MenuView;

fn main() -> Result<()> {
    let mut root: Box<dyn Component> = Box::new(MenuView::new());
    let root_printer = Printer::new(Rect::full_terminal()?, Style::default());

    terminal::enable()?;

    loop {
        if poll(Duration::from_millis(100))? {
            let e = read()?;

            match e {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => root.on_key(key),   
                    }
                }
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            }
        };

        terminal::clear()?;
        root.draw(&root_printer)?;
        terminal::flush()?;
    }

    terminal::disable()?;
    Ok(())
}
