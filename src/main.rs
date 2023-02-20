use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
};

mod lib;
mod state;
mod ui;
mod views;

use lib::{Component, Rect, Terminal};
use state::State;
use views::MenuView;

fn main() -> Result<()> {
    let mut terminal = Terminal { out: stdout() };
    let mut state = State::default();
    let mut root: Box<dyn Component<State>> = Box::new(MenuView::new());

    terminal.enable()?;

    state.redraw();

    loop {
        if poll(Duration::from_millis(250))? {
            let e = read()?;

            match e {
                Event::Key(key) => {
                    if key.code == KeyCode::Esc {
                        break;
                    }

                    root.key(key, &mut state);
                    state.redraw();
                }
                Event::Resize(_, _) => {
                    root.resize(Rect::full_terminal()?, &mut state);
                    state.redraw();
                }
                Event::Mouse(_) => {}
            }
        };

        if state.need_update {
            root.update(&mut state);
            state.redraw();
        }

        if state.need_redraw {
            terminal.out.queue(Clear(ClearType::All))?;
            root.draw(&mut terminal.out, &Rect::full_terminal()?, &state)?;
            terminal.out.flush()?;
        }
    }

    terminal.disable()?;
    Ok(())
}
