use std::{io::Write, time::Duration};

use crossterm::{
    event::{read, Event, KeyCode, poll}, Result, queue, terminal::{ClearType, Clear}, QueueableCommand,
};

use ui::{MainMenu, Component};

mod state;
mod ui;
mod app;

fn main() -> Result<()> {
    let mut app = app::App::new();
    app.enable_terminal()?;

    let mut root_component: Box<dyn Component> = Box::new(MainMenu::default());
    
    loop {
        if poll(Duration::from_millis(250))? {
            let e = read()?;

            match e {
                Event::Key(event) => {
                    if event.code == KeyCode::Esc {
                        break;
                    }

                    root_component.on_terminal_event(&mut app, e)
                },
                _ => root_component.on_terminal_event(&mut app, e),
            }
        };

        app.out.queue(Clear(ClearType::All))?;
        root_component.draw(&mut app)?;
        app.out.flush()?;
    }

    app.disable_terminal()?;
    Ok(())
}
