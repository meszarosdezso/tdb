pub struct Size {
    pub width: u64,
    pub height: u64,
}

impl Size {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }
}
