pub mod components;

use components::Component;
use components::text::Text;
use components::container::Container;
use components::size::Size;

use terminal_size::{Width, Height};

fn main() {
    let size = if let Some((Width(w), Height(h))) = terminal_size::terminal_size() {
        Size::new(w as u64, h as u64 - 1)
    } else {
        Size::new(80, 40)
    };

    let text = Text::new("Welcome to TDB!");
    let container = Container::fit().with_text(text);

    container.render(size);

    print!("\x1B[0m");
}
