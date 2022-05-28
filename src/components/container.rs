use super::*;
use crate::Inset;
use crate::Border;
use pixel::{Pixel, RGB};

pub struct Container {
    width: Option<usize>,
    height: Option<usize>,
    content: Option<Box<dyn Component>>,
    padding: Option<Inset>,
    color: Option<RGB>,
    border: Option<Border>,
}

impl Container {
    pub fn from_size(width: usize, height: usize) -> Self {
        Self {
            content: None, 
            width: Some(width),
            height: Some(height),
            padding: None,
            color: None,
            border: None,
        }
    }

    pub fn content<C: Component + 'static>(mut self, content: C) -> Self {
        self.content = Some(Box::new(content));
        self
    }

    pub fn padding(mut self, padding: Inset) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn color(mut self, color: RGB) -> Self {
        self.color = Some(color);
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }
}

impl Component for Container {
    fn build(&self, _context: RenderContext) -> BuiltComponent {
        let padding = &self.padding.clone().unwrap_or_default();
        
        let pl = padding.left;
        let pt = padding.top;
        let pr = padding.right;
        let pb = padding.bottom;

        let content = match &self.content {
            Some(content) => {
                let ctx = RenderContext { size: None };
                content.build(ctx)
            },
            None => BuiltComponent { buffer: vec![], size: Size::zero() }
        };

        let w = self.width.unwrap_or(content.size.width + pl + pr); 
        let h = self.height.unwrap_or(content.size.height + pt + pb); 

        let def = Pixel { content: 32, color: None, background_color: self.color.clone() };

        let mut buf = vec![def; w * h];

        if let Some(border) = &self.border {
            let pixel = Pixel { content: 32, background_color: Some(border.color.clone()), color: None };
            for x in 1..w - 1 {
                *buf.get_mut(x).unwrap() = pixel.clone();
                let idx = buf.len() - x as usize - 1;
                *buf.get_mut(idx).unwrap() = pixel.clone();
            }

            for y in 0..h {
                *buf.get_mut(y * w).unwrap() = pixel.clone();
                *buf.get_mut(y * w + 1).unwrap() = pixel.clone();
                *buf.get_mut(y * w + (w - 1)).unwrap() = pixel.clone();
                *buf.get_mut(y * w + (w - 2)).unwrap() = pixel.clone();
            }
        }

        for y in 0..content.size.height {
            for x in 0..content.size.width {
                let index = y * content.size.width + x;
                let pixel = content.buffer.get(index).unwrap().clone();

                let index = (y + pt) * w + (x + pl);
                *buf.get_mut(index).unwrap() = pixel;
            }
        }

        BuiltComponent { 
            buffer: buf,
            size: Size::new(w, h)
        }
    }
}
