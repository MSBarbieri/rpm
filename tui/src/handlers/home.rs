use super::{super::app::App};
use crate::event::Key;

const LARGE_SCROLL: u16 = 10;
const SMALL_SCROLL: u16 = 1;

pub fn handler(key: Key, app: &mut App) {
  match key {
    // k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    // k if common_key_events::down_event(k) => {
    //   app.windows_settings.home_scroll += SMALL_SCROLL;
    // }
    // k if common_key_events::up_event(k) => {
    //   if app.windows_settings.home_scroll > 0 {
    //     app.windows_settings.home_scroll -= SMALL_SCROLL;
    //   }
    // }
    k if k == app.config.key_bindings.next_page => {
      app.windows_settings.home_scroll += LARGE_SCROLL;
    }
    k if k == app.config.key_bindings.previous_page => {
      if app.windows_settings.home_scroll > LARGE_SCROLL {
        app.windows_settings.home_scroll -= LARGE_SCROLL;
      } else {
        app.windows_settings.home_scroll = 0;
      }
    }
    _ => {}
  }
}
