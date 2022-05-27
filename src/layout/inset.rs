#[derive(Clone, Debug, Default)]
pub struct Inset {
    pub left: u64,
    pub top: u64,
    pub right: u64,
    pub bottom: u64
}

impl Inset {
    pub fn symmetric(horizontal: u64, vertical: u64) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical
        }
    }
}
