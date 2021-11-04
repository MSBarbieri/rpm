use crate::event::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyBindings {
    pub back: Key,
    pub next_page: Key,
    pub previous_page: Key,
    pub manage_devices: Key,
    pub seek_backwards: Key,
    pub seek_forwards: Key,
    pub help: Key,
    pub shuffle: Key,
    pub repeat: Key,
    pub search: Key,
    pub submit: Key,
    pub copy_song_url: Key,
    pub copy_album_url: Key,
    pub audio_analysis: Key,
    pub basic_view: Key,
    pub add_item_to_queue: Key,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            back: Key::Char('q'),
            next_page: Key::Ctrl('d'),
            previous_page: Key::Ctrl('u'),
            manage_devices: Key::Char('d'),
            seek_backwards: Key::Char('<'),
            seek_forwards: Key::Char('>'),
            help: Key::Char('?'),
            shuffle: Key::Ctrl('s'),
            repeat: Key::Ctrl('r'),
            search: Key::Char('/'),
            submit: Key::Enter,
            copy_song_url: Key::Char('c'),
            copy_album_url: Key::Char('C'),
            audio_analysis: Key::Char('v'),
            basic_view: Key::Char('B'),
            add_item_to_queue: Key::Char('z'),
        }
    }
}
