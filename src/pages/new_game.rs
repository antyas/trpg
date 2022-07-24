use cursive::{Cursive, views::{SelectView, Dialog, EditView}, align::HAlign, traits::{Nameable, Resizable}};

use crate::{GameState, open_page, Page, content::races};

fn choise_race(s: &mut Cursive) {
    let mut race_switch = SelectView::new().h_align(HAlign::Center);
    let list = races();

    list.iter().enumerate()
        .for_each(|(i, x)| race_switch.add_item(x.creature_name.as_str(), i));

    race_switch.set_on_submit(move |s, i| {
        s.with_user_data(|gs: &mut GameState| gs.hero = list[*i].clone());
        s.pop_layer();
        input_name(s);
    });
    
    let race_dialog = Dialog::around(race_switch)
        .title("Выбор расы")
        .button("Назад", |s| { open_page(s, &Page::MainMenu) });

    s.add_layer(race_dialog);
}

fn input_name(s: &mut Cursive) {
    let name_input = EditView::new()
        .on_submit(|s: &mut Cursive, name: &str| {
            s.with_user_data(|gs: &mut GameState| gs.hero.name = name.to_string());
            s.pop_layer();
        })
        .with_name("name")
        .fixed_width(30);

    let name_dialog = Dialog::around(name_input)
        .title("Введите имя")
        .button("Назад", |s| { open_page(s, &Page::MainMenu) });

    s.add_layer(name_dialog);
}

pub fn page(s: &mut Cursive) {
    choise_race(s);
}