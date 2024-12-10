use colored::*;
use term_size;

pub const DARK_GREEN: (u8, u8, u8) = (0, 60, 0);
pub const DARK_RED: (u8, u8, u8) = (80, 0, 0);

pub enum ChangeType {
    Addition,
    Deletion,
}

impl ChangeType {
    pub fn format_line(&self, content: &str) -> String {
        // Get the terminal width, fallback to 80 if not available
        let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
        let content_length = content.len() + 1; // +1 for the '+' or '-'

        let padding_length = width.saturating_sub(content_length);

        let padding = " ".repeat(padding_length);
        let content_trimmed = &content.trim_end();

        match self {
            ChangeType::Addition => {
                let (r, g, b) = DARK_GREEN;
                format!("+{}{}", content_trimmed, padding)
                    .as_str()
                    .on_truecolor(r, g, b)
                    .truecolor(255, 255, 255) // White text
                    .bold()
                    .to_string()
            }
            ChangeType::Deletion => {
                let (r, g, b) = DARK_RED;
                format!("-{}{}", content_trimmed, padding)
                    .as_str()
                    .on_truecolor(r, g, b)
                    .truecolor(255, 255, 255) // White text
                    .bold()
                    .to_string()
            }
        }
    }
}
