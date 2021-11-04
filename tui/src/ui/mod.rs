use crate::config::Theme;
mod help_page;
pub use help_page::*;

mod home_page;
pub use home_page::*;

mod page;
pub use page::*;
mod search_page;
pub use search_page::*;

use tui::style::Style;

pub fn get_color((is_active, is_hovered): (bool, bool), theme: &Theme) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(theme.selected),
        (false, true) => Style::default().fg(theme.hovered),
        _ => Style::default().fg(theme.inactive),
    }
}
