use std::io::Stdout;

use crossterm::{event::KeyEvent, Result};

use super::Rect;

pub trait BaseComponent<I, O: Default> {
    fn draw(&self, _out: &mut Stdout, _rect: &Rect) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, _rect: Rect, _data: I) {}
    fn update(&mut self, _data: I) -> O {
        O::default()
    }
    fn key(&mut self, _key: KeyEvent) -> O {
        O::default()
    }
}

pub trait Component<S> {
    fn draw(&self, _out: &mut Stdout, _rect: &Rect, _state: &S) -> Result<()> {
        Ok(())
    }
    fn resize(&mut self, _rect: Rect, _state: &S) {}
    fn update(&mut self, _state: &mut S) {}
    fn key(&mut self, _key: KeyEvent, _state: &mut S) {}
}
