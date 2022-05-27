use super::{Component, Pixel, RGB, Size};

pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self { content: content.to_owned() }
    }
}

impl Component for Text {
    fn to_buf(&self, parent_size: Size) -> Vec<Pixel> {
        let max_width = self.content.split('\n').map(|l| l.len()).max().unwrap_or_default();

        let w = if parent_size.width > 0 { parent_size.width as usize } else { max_width };
        let h = parent_size.height as usize;

        let mut buf = Vec::with_capacity((w * h) as usize);
       
        for c in self.content.chars() {
            match c {
                '\n' => {
                    let rem = buf.len() % w;
                    for _ in 0..w - rem {
                        buf.push(Pixel::empty());
                    }
                },
                _ => {
                    let pixel = Pixel { content: c as u8, color: Some(RGB(255, 255, 255)), background_color: None };
                    buf.push(pixel)
                }
            }

        }

        buf
    }
}
