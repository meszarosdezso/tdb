use super::*;
use text::Text;
use crate::Inset;

pub struct Container {
    width: Option<u64>,
    height: Option<u64>,
    content: Option<Text>,
    padding: Option<Inset>,
}

impl Container {
    pub fn from_size(width: u64, height: u64) -> Self {
        Self {
            content: None,
            width: Some(width),
            height: Some(height),
            padding: None
        }
    }

    pub fn fit() -> Self {
        Self { content: None, width: None, height: None, padding: None }
    }

    pub fn text(mut self, content: Text) -> Self {
        self.content = Some(content);
        self
    }

    pub fn padding(mut self, padding: Inset) -> Self {
        self.padding = Some(padding);
        self
    }
}

impl Component for Container {
    fn render(&self, parent_size: Size) {
        let h = self.height.unwrap_or(parent_size.height);
        let w = self.width.unwrap_or(parent_size.width);

        for j in 0..h {
            for i in 0..w {
                let mut c = match &self.padding {
                    Some(Inset { left, right, ..}) if i < *left || i > (w - *right - 1) => ' ',
                    Some(Inset { top, bottom, .. }) if j < *top || j > (h - *bottom - 1) => ' ',
                    Some(Inset { left, top, .. }) => match &self.content {
                        Some(content) => { 
                            let idx = ((j - *top) * w + (i - *left)) as usize;
                            content.content.chars().nth(idx).unwrap_or(' ')
                        },
                        None => ' ',
                    },
                    None => match &self.content {
                        Some(content) => { 
                            let idx = (j * w + i ) as usize;
                            content.content.chars().nth(idx).unwrap_or(' ')
                        },
                        None => ' ',
                    },
                };

                if c == '\n' { c = ' ' }

                print!("\x1B[48;2;20;24;26m{}", c);
            }
            println!("\x1B[0m");
        }
    }
}
