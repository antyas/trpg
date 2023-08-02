use crossterm::event::KeyEvent;

use crate::lib::{Component, Indents, List, Text, Printer, Style};

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
    fn draw(&self, printer: &Printer) -> crossterm::Result<()> {
        let info_height = self.bottom_info.height();
        let menu_rect = printer.rect.with_margin(Indents::new(0, 0, info_height, 0));
        let info_rect = printer.rect.with_margin(Indents::new(info_height, 0, 0, 0));

        let menu_printer = Printer::new(menu_rect, Style::default());
        self.main_menu.draw(&menu_printer)?;

        let info_printer = Printer::new(info_rect, Style::default());
        self.bottom_info.draw(&info_printer)?;

        Ok(())
    }

    fn on_key(&mut self, key: KeyEvent) {
        self.main_menu.on_key(key);
    }
}
