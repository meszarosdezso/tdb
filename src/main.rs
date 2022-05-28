pub mod components;
pub mod layout;
pub mod utils;

use components::Component; 
use components::pixel::RGB;
use components::text::Text;
use components::container::Container;
use layout::size::Size;
use layout::inset::Inset;
use layout::border::Border;
use layout::context::RenderContext;
use utils::characters::CLEAR;

use terminal_size::{Width, Height};

fn main() {
    eprint!("{}", CLEAR);
    let size = if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
        Size::new(w as usize, h as usize - 1)
    } else {
        Size::new(80, 40)
    };

    let text = Text::new("Graphic design\n\nis my passion! (:").color(RGB(0,255,100));

    let header = Container::from_size(size.width - 8, 8)
        .padding(Inset::symmetric(4, 2))
        .color(RGB(140, 8, 152))
        .border(Border::thin())
        .content(text);

    let root = Container::from_size(size.width, size.height)
        .color(RGB(20, 24, 26))
        .padding(Inset::symmetric(4, 2))
        .content(header)
        .build(RenderContext { size: Some(size) });

    for pixel in root.buffer {
        pixel.render();
    }

    println!("\x1B[0m");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
}
