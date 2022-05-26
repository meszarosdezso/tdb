use super::*;
use text::Text;

pub struct Container {
    width: Option<u64>,
    height: Option<u64>,
    content: Option<Text>
}

impl Container {
    pub fn from_size(width: u64, height: u64) -> Self {
        Self { content: None, width: Some(width), height: Some(height) }
    }

    pub fn fit() -> Self {
        Self { content: None, width: None, height: None }
    }

    pub fn with_text(mut self, content: Text) -> Self {
        self.content = Some(content);
        self
    }
}

impl Component for Container {
    fn render(&self, parent_size: Size) {
        let h = self.height.unwrap_or(parent_size.height);
        let w = self.width.unwrap_or(parent_size.width);

        for j in 0..h {
            for i in 0..w {
                let c = match &self.content {
                    Some(content) => content.content.chars().nth((j * w + i) as usize).unwrap_or(' '),
                    None => ' ',
                };

                print!("\x1B[48;2;20;24;26m{}", c);
            }
            println!("\x1B[0m");
        }
    }
}
