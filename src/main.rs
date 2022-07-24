use std::collections::HashMap;

use content::races;
use cursive::{
    align::HAlign,
    event,
    menu,
    traits::{Nameable, Resizable},
    views::{Dialog, EditView, ListView, SelectView, SliderView, TextView},
    Cursive, With, view::IntoBoxedView,
};
use system::Creature;

mod system;
mod content;
mod pages;

enum Page {
    Quit,
    NewGame,
    MainMenu,
}

enum GameTabs {
    Main,
    Inventory,
    Character,
}

#[derive(Default)]
struct GameState {
    hero: Creature,
}

fn main_menu_page(s: &mut Cursive) {
    let mut menu_select = SelectView::new().h_align(HAlign::Center);
    menu_select.add_item(" Новая игра ", Page::NewGame);
    menu_select.add_item(" Выход ", Page::Quit);
    menu_select.set_on_submit(|s, item| open_page(s, item));
    s.add_layer(menu_select);
}

fn open_page(s: &mut Cursive, page: &Page) {
    s.pop_layer();

    match page {
        Page::Quit => s.quit(),
        Page::NewGame => pages::new_game::page(s),
        Page::MainMenu => main_menu_page(s),
    };
}

fn switch_view<T>(s: &mut Cursive, view: T)
where
    T: IntoBoxedView
{
    s.pop_layer();
    s.add_layer(view)
}

fn main() {
    let mut siv = cursive::default();

    siv.load_toml(include_str!("../theme.toml")).unwrap();

    siv.add_global_callback('q', |s| s.quit());

    siv.set_user_data(GameState::default());

    open_page(&mut siv, &Page::MainMenu);

    siv.run();
}
