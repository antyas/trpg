pub mod terminal;
mod components;
mod layout;
mod style;
mod traits;
mod printer;

pub use components::*;
pub use layout::*;
pub use style::*;
pub use traits::*;
pub use printer::*;

pub struct State<T> {
    value: T,
}

impl<T> State<T> {
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}
