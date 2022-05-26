use super::{Component, Size};

pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self { content: content.to_owned() }
    }
}

impl Component for Text {
    fn render(&self, parent_size: Size) {
        println!("{}", self.content);
    }
}
