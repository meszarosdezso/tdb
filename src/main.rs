pub mod components;
pub mod layout;
pub mod utils;

use components::{Component, RGB};
use components::text::Text;
use components::container::Container;
use components::size::Size;
use layout::inset::Inset;
use utils::characters::CLEAR;

use terminal_size::{Width, Height};

fn main() {
    eprint!("{}", CLEAR);
    let size = if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
        Size::new(w as u64, h as u64 - 1)
    } else {
        Size::new(80, 40)
    };

    let text = Text::new("Welcome to TDB!\n\nHow are you?");

//    let header = Container::from_size(10, 6)
//        .color(RGB(240, 248, 252))
//        .content(text);

    let root = Container::from_size(size.width, size.height)
        .color(RGB(20, 24, 26))
        .padding(Inset::symmetric(4, 2))
        .content(text);

    let buf = root.to_buf(size);

    for pixel in buf {
        pixel.render();
    }

    print!("\x1B[0m");
}
