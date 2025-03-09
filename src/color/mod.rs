#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    RGB {r: u8, g: u8, b: u8},
}

impl Color {
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::RGB { r, g, b }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::RGB { r, g, b } => write!(f, "rgb({r}, {g}, {b})"),
        }
    }
}


