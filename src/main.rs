pub mod components;
pub mod layout;
pub mod utils;

use components::{Component, RGB};
use components::text::Text;
use components::container::Container;
use components::size::Size;
use layout::inset::Inset;
use layout::border::Border;
use utils::characters::CLEAR;

use terminal_size::{Width, Height};

fn main() {
    eprint!("{}", CLEAR);
    let size = if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
        Size::new(w as u64, h as u64 - 1)
    } else {
        Size::new(80, 40)
    };

    let text = Text::new("Graphic design\n\nis my passion! (:").color(RGB(0,255,100));

    let header = Container::from_size(size.width - 8, 7)
        .padding(Inset::symmetric(4, 2))
        .color(RGB(140, 8, 152))
        .border(Border::thin())
        .content(text);

    let root = Container::from_size(size.width, size.height)
        .color(RGB(20, 24, 26))
        .padding(Inset::symmetric(4, 2))
        .content(header);

    let buf = root.to_buf(size);

    for pixel in buf {
        pixel.render();
    }

    print!("\x1B[0m");
}
