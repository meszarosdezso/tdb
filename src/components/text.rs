use super::{Component, Pixel, RGB, Size};

pub struct Text {
    pub content: String,
    pub color: Option<RGB>,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self { content: content.to_owned(), color: None }
    }

    pub fn color(mut self, color: RGB) -> Self {
        self.color = Some(color);
        self
    }
}

impl Component for Text {

    fn get_size(&self) -> Size {
        let lines = self.content.split('\n');
        let h = lines.clone().count();
        let w = &lines.map(|l| l.len()).max().unwrap_or_default();

        Size::new(*w as u64, h as u64)
    }

    fn to_buf(&self, parent_size: Size) -> Vec<Pixel> {
        let max_width = self.content.split('\n').map(|l| l.len()).max().unwrap_or_default();

        let w = max_width; 
        let h = parent_size.height as usize;
        println!("Width: {w}, Height: {h}");

        let mut buf = Vec::with_capacity((w * h) as usize);
        let color = self.color.clone().unwrap_or(RGB(255, 255, 255));
       
        for c in self.content.chars() {
            match c {
                '\n' => {
                    let rem = buf.len() % w;
                    for _ in 0..w - rem {
                        buf.push(Pixel::empty());
                    }
                },
                _ => {
                    let pixel = Pixel { content: c as u8, color: Some(color.clone()), background_color: None };
                    buf.push(pixel);
                }
            };
        }

        buf
    }
}
