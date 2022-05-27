use super::*;
use crate::Inset;

pub struct Container {
    width: Option<u64>,
    height: Option<u64>,
    content: Vec<Pixel>,
    padding: Option<Inset>,
    color: Option<RGB>,
}

impl Container {
    pub fn from_size(width: u64, height: u64) -> Self {
        Self {
            content: Vec::new(),
            width: Some(width),
            height: Some(height),
            padding: None,
            color: None,
        }
    }

    pub fn content<C: Component>(mut self, content: C) -> Self {
        self.content = content.to_buf(Size::new(self.width.unwrap_or_default(), self.height.unwrap_or_default()));
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
}

impl Component for Container {
    fn to_buf(&self, parent_size: Size) -> Vec<Pixel> {
        let h = self.height.unwrap_or(parent_size.height);
        let w = self.width.unwrap_or(parent_size.width);

        let mut buf = Vec::with_capacity((w * h) as usize);
        let def = Pixel { content: 32, color: None, background_color: self.color.clone() };

        for j in 0..h {
            for i in 0..w {
                let p = match &self.padding {
                    Some(Inset { left, right, ..}) if i < *left || i > (w - *right - 1) => def.clone(),
                    Some(Inset { top, bottom, .. }) if j < *top || j > (h - *bottom - 1) => def.clone(),
                    Some(Inset { left, top, .. }) => {
                        let idx = ((j - *top) * w + (i - *left)) as usize;
                        self.content.get(idx).unwrap_or(&def).clone()
                    },
                    None => {
                        let idx = (j * w + i) as usize;
                        self.content.get(idx).unwrap_or(&def).clone()
                    },
                };

                buf.push(p);
            }
        }

        buf
    }
}
