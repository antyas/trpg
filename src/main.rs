use std::collections::HashMap;

use cursive::{
    align::HAlign,
    event,
    menu,
    traits::{Nameable, Resizable},
    views::{Dialog, EditView, ListView, SelectView, SliderView, TextView},
    Cursive, With,
};

mod system;

enum MainMenuItems {
    Quit,
    NewGame,
}

enum GameTabs {
    Main,
    Inventory,
    Character,
}

fn start_new_game(s: &mut Cursive) {
    s.add_layer(Dialog::around(TextView::new("New game started")).button("Quit", |s| s.quit()));
}

fn main() {
    let mut siv = cursive::default();

    siv.load_toml(include_str!("../theme.toml")).unwrap();

    siv.add_global_callback('q', |s| s.quit());

    let mut time_select = SelectView::new().h_align(HAlign::Center);
    time_select.add_item(" New game ", MainMenuItems::NewGame);
    time_select.add_item(" Exit ", MainMenuItems::Quit);

    time_select.set_on_submit(|s, item| {
        s.pop_layer();
        
        match item {
            MainMenuItems::NewGame => start_new_game(s),
            MainMenuItems::Quit => s.quit(),
        }
    });

    siv.add_layer(time_select);

    siv.run();
}
