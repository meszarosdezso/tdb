use super::{Component, Size, BuiltComponent, RenderContext};
use crate::components::pixel::{Pixel, RGB};

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

// Context { size: None }
// => [h,e,l,l,o, ,w,o,r,l,d]
//
// Context { size: (5, 5) }
// => [h,e,l,l,o,
//      ,w,o,r,l,
//     d, , , , ,
//      , , , , ,
//      , , , , ,]

impl Component for Text {
    fn build(&self, context: RenderContext) -> BuiltComponent {
        let size = match context.size {
            Some(size) => size,
            None => {
                let lines = self.content.split('\n');
                let height = &lines.clone().count();
                let width = lines.map(|l| l.len()).max().unwrap_or_default();

                Size::new(width, *height)
            }
        };

        println!("Text size: {size:?}");

        let mut buffer = Vec::with_capacity(size.width * size.height);
        let color = self.color.clone().unwrap_or(RGB(255, 255, 255));

        let mut prev_c = ' ';
        for c in self.content.chars() {
            match c {
                '\n' => {
                    let rem = buffer.len() % size.width;
                    if rem > 0 || prev_c == '\n' {
                        let empty_count = size.width - rem;

                        for _ in 0..empty_count {
                            buffer.push(Pixel::empty());
                        }
                    }
                },
                _ => {
                    let pixel = Pixel { content: c as u8, color: Some(color.clone()), background_color: None };
                    buffer.push(pixel);
                }
            };
            prev_c = c;
        }

        let free = buffer.capacity() - buffer.len();
        for _ in 0..free {
            buffer.push(Pixel::empty());
        }

        BuiltComponent { buffer, size }
    }
}
