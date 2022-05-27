use super::*;
use crate::Inset;
use crate::Border;

pub struct Container {
    width: Option<u64>,
    height: Option<u64>,
    content: Option<Box<dyn Component>>,
    padding: Option<Inset>,
    color: Option<RGB>,
    border: Option<Border>,
}

impl Container {
    pub fn from_size(width: u64, height: u64) -> Self {
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
    fn get_size(&self) -> Size {
        let w = self.width.unwrap_or(0);
        let h = self.height.unwrap_or(0);
        Size::new(w, h)
    }

    fn to_buf(&self, _parent_size: Size) -> Vec<Pixel> {
        let padding = &self.padding.clone().unwrap_or_default();
        
        let pl = padding.left;
        let pt = padding.top;
        let pr = padding.right;
        let pb = padding.bottom;

        let content_size = if let Some(content) = &self.content { content.get_size() } else { Size::zero() };

        let h = self.height.unwrap_or(content_size.height + pt + pb); 
        let w = self.width.unwrap_or(content_size.width + pl + pr); 

        let def = Pixel { content: 32, color: None, background_color: self.color.clone() };

        let mut buf = vec![def; (w * h) as usize];

        if let Some(border) = &self.border {
            let pixel = Pixel { content: 32, background_color: Some(border.color.clone()), color: None };
            for x in 1..w - 1 {
                *buf.get_mut(x as usize).unwrap() = pixel.clone();
                let idx = buf.len() - x as usize - 1;
                *buf.get_mut(idx).unwrap() = pixel.clone();
            }

            for y in 0..h {
                *buf.get_mut((y * w) as usize).unwrap() = pixel.clone();
                *buf.get_mut(((y * w) + 1) as usize).unwrap() = pixel.clone();
                *buf.get_mut((y * w + (w - 1)) as usize).unwrap() = pixel.clone();
                *buf.get_mut((y * w + (w - 2)) as usize).unwrap() = pixel.clone();
            }
        }

        if let Some(content) = &self.content {
            let content_buf = content.to_buf(Size::new(w - pl - pr, h - pt - pb));

            for y in 0..content_size.height {
                for x in 0..content_size.width {
                    let index = (y * content_size.width + x) as usize;
                    let pixel = content_buf.get(index).unwrap().clone();

                    let index = ((y + pt) * w + (x + pl)) as usize;
                    *buf.get_mut(index).unwrap() = pixel;
                }
            }
        }
        buf
    }
}
