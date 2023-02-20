#[derive(Default)]
pub struct State {
    pub need_redraw: bool,
    pub need_update: bool,
    pub bottom_info: Vec<String>,
}

impl State {
    pub fn update(&mut self) {
        self.need_update = true;
    }

    pub fn redraw(&mut self) {
        self.need_redraw = true;
    }
}
