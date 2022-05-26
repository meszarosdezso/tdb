pub mod text;
pub mod container;
pub mod size;

use size::Size;

pub(crate) trait Component {
    fn render(&self, parent_size: Size);
}
