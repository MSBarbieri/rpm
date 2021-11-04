use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Behavior {
    pub tick_rate_milliseconds: u64,
    pub set_window_title: bool,
    pub show_update_indicator: bool,
    pub show_release_candidate: bool,
    pub enforce_wide_search_bar: bool,
    pub show_loading_indicator: bool
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior {
            set_window_title: true,
            tick_rate_milliseconds: 250,
            show_update_indicator: true,
            show_release_candidate: false,
            enforce_wide_search_bar: false,
            show_loading_indicator: true
        }
    }
}
