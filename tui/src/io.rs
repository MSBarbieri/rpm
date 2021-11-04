use crossterm::event::{poll, read, Event as CEvent, KeyCode, KeyEvent, KeyModifiers};
use std::{io::stdin, sync::mpsc, thread, time::Duration};

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<KeyEvent>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: KeyEvent,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            },
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let tx2 = tx.clone();
        thread::spawn(move || {
            loop {
                // `poll()` waits for an `Event` for a given time period
                if poll(config.tick_rate).unwrap() {
                    // It's guaranteed that the `read()` won't block when the `poll()`
                    // function returns `true`
                    
                    match read().unwrap() {
                        CEvent::Key(event) => {
                            tx.send(event);
                        }
                        CEvent::Mouse(e) => {
                            println!("mouse event: {:?}", e);
                        }
                        CEvent::Resize(width, height) => {
                            println!("New size {}x{}", width, height);
                        }
                    };
                } else {
                    // Timeout expired and no `Event` is available
                }
            }
        });

        Events { rx }
    }

    pub fn next(&self) -> Result<KeyEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}
