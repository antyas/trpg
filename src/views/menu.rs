use crossterm::event::KeyEvent;

use crate::{
    lib::{Component, Indents, List, Text, Printer},
    state::State,
};

pub struct MenuView {
    main_menu: List,
    bottom_info: Text,
}
impl MenuView {
    pub fn new() -> Self {
        Self {
            main_menu: List::new(vec![
                "Загрузить".into(),
                "Новая игра".into(),
                "Выход".into(),
            ]),
            bottom_info: Text::new(vec!["Выберите пункт меню".into()]),
        }
    }
}

impl Component for MenuView {
    fn draw(&self, printer: Printer, app: &mut app) -> crossterm::Result<()> {
        let info_height = self.bottom_info.height();
        let menu_rect = rect.with_margin(Indents::new(0, 0, info_height, 0));
        let info_rect = rect.with_margin(Indents::new(info_height, 0, 0, 0));

        self.main_menu.draw(out, &menu_rect)?;
        self.bottom_info.draw(out, &info_rect)?;
        Ok(())
    }

    fn key(&mut self, key: KeyEvent, _: &mut State) {
        self.main_menu.key(key);
    }
}
