use serde::{Deserialize, Serialize};
use tui::style::Color;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    pub analysis_bar: Color,
    pub analysis_bar_text: Color,
    pub active: Color,
    pub banner: Color,
    pub error_border: Color,
    pub error_text: Color,
    pub hint: Color,
    pub hovered: Color,
    pub inactive: Color,
    pub selected: Color,
    pub text: Color,
    pub header: Color,
}
impl Default for Theme {
    fn default() -> Self {
        Theme {
            analysis_bar: Color::LightCyan,
            analysis_bar_text: Color::Reset,
            active: Color::Cyan,
            banner: Color::LightCyan,
            error_border: Color::Red,
            error_text: Color::LightRed,
            hint: Color::Yellow,
            hovered: Color::Magenta,
            inactive: Color::Gray,
            selected: Color::LightCyan,
            text: Color::Reset,
            header: Color::Reset,
        }
    }
}
