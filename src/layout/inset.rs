#[derive(Clone, Debug, Default)]
pub struct Inset {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize
}

impl Inset {
    pub fn symmetric(horizontal: usize, vertical: usize) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical
        }
    }

    pub fn new(left: Option<usize>, top: Option<usize>, right: Option<usize>, bottom: Option<usize>) -> Self {
        Self {
            left: left.unwrap_or_default(),
            top: top.unwrap_or_default(),
            right: right.unwrap_or_default(),
            bottom: bottom.unwrap_or_default(),
        }
    }
}
