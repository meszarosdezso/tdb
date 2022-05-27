use crate::RGB;

pub enum BorderStyle {
    FULL,
    THIN
}

pub struct Border {
    pub style: BorderStyle,
    pub color: RGB,
}

impl Border {
    pub fn full() -> Self {
        Self { style: BorderStyle::FULL, color: Default::default() }
    }

    pub fn thin() -> Self {
        Self { style: BorderStyle::THIN, color: Default::default() }
    }
}
