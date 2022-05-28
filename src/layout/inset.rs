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
}
