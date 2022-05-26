pub mod components;
pub mod layout;

use components::Component;
use components::text::Text;
use components::container::Container;
use components::size::Size;
use layout::inset::Inset;

use terminal_size::{Width, Height};

fn main() {
    println!("\x1B[2J");

    let size = if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
        Size::new(w as u64, h as u64 - 1)
    } else {
        Size::new(80, 40)
    };

    let text = Text::new("Welcome to TDB!\n\nHow was your day? Please input some words about your day below!");
    let container = Container::fit()
        .text(text)
        .padding(Inset::symmetric(4, 2));

    container.render(size);

    print!("\x1B[0m");
}
