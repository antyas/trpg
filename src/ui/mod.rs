mod main_menu;

pub use main_menu::MainMenu;

use crossterm::{Result, event::Event, style::Color};
use crate::app::App;

pub trait Component {
    fn draw(&self, app: &mut App) -> Result<()>;
    fn on_state_update(&mut self, app: &mut App);
    fn on_terminal_event(&mut self, app: &mut App, event: Event);
}

pub const TEXT_PRIMARY_COLOR: Color = Color::White;
pub const TEXT_FOCUS_COLOR: Color = Color::Green;