mod app;
mod config;
mod event;
mod handlers;
mod router;
mod ui;

use crate::config::Config;
use manager::App as RealApp;

use crate::{
    app::App,
    event::{Event, Key},
    router::ActiveBlock,
};
use anyhow::Result;
use backtrace::Backtrace;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
    ExecutableCommand,
};
use std::{io::Stdout, panic::PanicInfo};
use tui::{backend::CrosstermBackend, Terminal};

pub fn panic_hook(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        disable_raw_mode().unwrap();
        execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            )),
            DisableMouseCapture
        )
        .unwrap();
    }
}

fn create_terminal(app: &App) -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    if app.config.behavior.set_window_title {
        backend.execute(SetTitle(app.title.clone()))?;
    }
    Ok(Terminal::new(backend)?)
}

fn close_terminal() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn resize_terminal_window(app: &mut App, terminal: &Terminal<CrosstermBackend<Stdout>>) {
    if let Ok(size) = terminal.size() {
        if app.windows_settings.size != size {
            app.windows_settings.help_menu_max_lines = 0;
            app.windows_settings.help_menu_offset = 0;
            app.windows_settings.help_menu_page = 0;

            app.windows_settings.size = size;

            if app.windows_settings.size.height > 8 {
                app.windows_settings.help_menu_max_lines = app.windows_settings.size.height - 8;
            } else {
                app.windows_settings.help_menu_max_lines = 0;
            }
        }
    }
}

pub async fn start_tui(_: &mut RealApp) -> Result<()> {
    let app = &mut (App::new(Config::default())?);

    let mut terminal = create_terminal(app)?;
    while !app.should_quit() {
        resize_terminal_window(app, &terminal);

        let route = app.router.get_current_route();
        terminal.draw(|f| match route.active_block {
            ActiveBlock::HelpMenu => {
                ui::help_page(&app, f);
            }
            _ => {
                ui::home_page(&app, f);
            }
        })?;

        match app.events.next()? {
            Event::Input(key) => {
                match key {
                    Key::Ctrl('c') => app.close(),
                    _ => handlers::handle_app(key, app),
                };
            }
            Event::Tick => {
                app.update_on_tick();
            }
        }
    }
    close_terminal()?;
    Ok(())
}
