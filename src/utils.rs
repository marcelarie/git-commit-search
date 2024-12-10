use colored::*;

pub const DARK_GREEN: (u8, u8, u8) = (0, 60, 0);
pub const DARK_RED: (u8, u8, u8) = (80, 0, 0);

pub enum ChangeType {
    Addition,
    Deletion,
}

impl ChangeType {
    pub fn format_line(&self, content: &str) -> String {
        match self {
            ChangeType::Addition => {
                let (r, g, b) = DARK_GREEN;
                format!("+{}", content)
                    .as_str()
                    .on_truecolor(r, g, b)
                    .truecolor(255, 255, 255) // White text
                    .bold()
                    .to_string()
            }
            ChangeType::Deletion => {
                let (r, g, b) = DARK_RED;
                format!("-{}", content)
                    .as_str()
                    .on_truecolor(r, g, b)
                    .truecolor(255, 255, 255) // White text
                    .bold()
                    .to_string()
            }
        }
    }
}
