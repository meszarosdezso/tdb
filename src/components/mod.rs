pub mod text;
pub mod container;
pub mod size;

use size::Size;

#[derive(Debug, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

impl Default for RGB {
    fn default() -> Self { RGB(255, 255, 255) }
}

#[derive(Debug, Clone)]
pub struct Pixel {
    content: u8,
    color: Option<RGB>,
    background_color: Option<RGB>,
}

impl Pixel {
    pub fn empty() -> Self {
        Self { content: 32, color: None, background_color: None }
    }

    pub fn render(&self) {
        let mut s = String::new();

        if let Some(color) = &self.background_color {
            s.push_str(format!(
                    "\x1B[48;2;{};{};{}m",
                    color.0,
                    color.1,
                    color.2,
                ).as_str()
            );
        }
        if let Some(color) = &self.color {
            s.push_str(format!(
                    "\x1B[38;2;{};{};{}m",
                    color.0,
                    color.1,
                    color.2,
                ).as_str()
            );
        }

        s.push(self.content as char);

        print!("{s}");
    }
}

pub trait Component {
    fn get_size(&self) -> Size;
    fn to_buf(&self, parent_size: Size) -> Vec<Pixel>;
}
