use super::{Component, Size, BuiltComponent, RenderContext};

#[derive(Default)]
pub struct List {
    pub content: Vec<Box<dyn Component>>
}

impl List {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<C: Component + 'static>(mut self, component: C) -> Self {
        self.content.push(Box::new(component));
        self
    }
}

impl Component for List {
    fn build(&self, context: RenderContext) -> BuiltComponent {
        let components = self.content.iter().map(|c| c.build(context.clone()));
        let ctx_size = context.clone().size.unwrap_or_default();

        let (w, h) = components.clone().fold((ctx_size.width, 0), |(mut w, mut h), c| {
            w = std::cmp::max(w, c.size.width);
            h = std::cmp::min(ctx_size.height, h + c.size.height);
            (w, h)
        });

        let size = Size::new(w, h);
        println!("List size: {size:?}, context: {context:?}");

        let mut buffer = Vec::with_capacity(w * h);

        for c in components {
            for pixel in c.buffer {
                buffer.push(pixel.clone());
            }
        }

        BuiltComponent { size, buffer }
    }
}
