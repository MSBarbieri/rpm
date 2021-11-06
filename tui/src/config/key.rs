use crate::event::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyBindings {
    pub back: Key,
    pub help: Key,
    pub submit: Key,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            back: Key::Char('q'),
            help: Key::Char('?'),
            submit: Key::Enter,
        }
    }
}
