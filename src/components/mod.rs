pub mod container;
pub mod list;
pub mod pixel;
pub mod text;

use crate::{RenderContext, Size};
use pixel::Pixel;

pub struct BuiltComponent {
    pub size: Size,
    pub buffer: Vec<Pixel>,
}

pub trait Component {
    fn build(&self, context: RenderContext) -> BuiltComponent;
}
