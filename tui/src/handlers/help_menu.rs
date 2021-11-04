
use crate::{app::App, event::Key};

#[derive(PartialEq)]
enum Direction {
  Up,
  Down,
}

pub fn handler(key: Key, app: &mut App) {
  match key {
    Key::Ctrl('d') => {
      move_page(Direction::Down, app);
    }
    Key::Ctrl('u') => {
      move_page(Direction::Up, app);
    }
    _ => {}
  };
}

fn move_page(direction: Direction, app: &mut App) {
  if direction == Direction::Up {
    if app.windows_settings.help_menu_page > 0 {
      app.windows_settings.help_menu_page -= 1;
    }
  } else if direction == Direction::Down {
    app.windows_settings.help_menu_page += 1;
  }
  app.calculate_help_menu_offset();
}
