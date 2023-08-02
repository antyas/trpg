use crossterm::Result;

pub struct Indents {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Indents {
    pub fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

pub enum Alignment {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

/// (width, height)
pub type Size = (u16, u16);

#[derive(Clone)]
pub struct Rect {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

impl Rect {
    pub fn new(x1: u16, y1: u16, x2: u16, y2: u16) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn size(&self) -> Size {
        (self.x2.saturating_sub(self.x1), self.y2.saturating_sub(self.y1))
    }

    pub fn full_terminal() -> Result<Self> {
        let (w, h) = crossterm::terminal::size()?;
        Ok(Self::new(0, 0, w, h))
    }

    pub fn with_margin(&self, margin: Indents) -> Self {
        Self {
            x1: self.x1.saturating_sub(margin.left),
            x2: self.x2.saturating_add(margin.right),
            y1: self.y1.saturating_sub(margin.top),
            y2: self.y2.saturating_add(margin.bottom),
        }
    }

    pub fn with_padding(&self, padding: Indents) -> Self {
        Self {
            x1: self.x1.saturating_add(padding.left),
            x2: self.x2.saturating_sub(padding.right),
            y1: self.y1.saturating_add(padding.top),
            y2: self.y2.saturating_sub(padding.bottom),
        }
    }

    pub fn with_align(&self, position: Alignment, new_size: Size) -> Self {
        let (nw, nh) = new_size;
        let (sw, sh) = self.size();
        let (offset_w, offset_h) = (sw.saturating_sub(nw), sh.saturating_sub(nh));
        let (half_w, half_h) = (offset_w / 2, offset_h / 2);
        let (left, right) = (offset_w - half_w, half_w);
        let (top, bottom) = (offset_h - half_h, half_h);

        let padding = match position {
            Alignment::TopLeft => Indents::new(0, offset_w, offset_h, 0),
            Alignment::Top => Indents::new(0, right, offset_h, left),
            Alignment::TopRight => Indents::new(0, 0, offset_h, offset_w),
            Alignment::Left => Indents::new(top, offset_w, bottom, 0),
            Alignment::Center => Indents::new(top, right, bottom, left),
            Alignment::Right => Indents::new(top, 0, bottom, offset_w),
            Alignment::BottomLeft => Indents::new(offset_h, offset_w, 0, 0),
            Alignment::Bottom => Indents::new(offset_h, right, 0, left),
            Alignment::BottomRight => Indents::new(offset_h, 0, 0, offset_w),
        };

        self.with_padding(padding)
    }
}
