#![warn(bare_trait_objects)]
mod app;
mod cli;
mod config;
mod event;
mod router;
mod ui;
mod handlers;

use crate::{app::App, event::{Event, Key}, router::{ActiveBlock, RouteId}};
use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use clap::{App as ClapApp, Arg, Shell};
use config::Config as CliConfig;
use core::cmp::max;
use core::cmp::min;
use crossterm::cursor::MoveTo;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
    ExecutableCommand,
};
use std::{
    io::Stdout,
    panic::{self, PanicInfo},
};
use tui::{backend::CrosstermBackend, Terminal};

fn panic_hook(info: &PanicInfo<'_>) {
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

pub fn create_terminal(app: &App) -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);

    if app.config.behavior.set_window_title {
        backend.execute(SetTitle(app.title.clone()))?;
    }

    Ok(Terminal::new(backend)?)
}

fn close_application() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    let title = String::from(r#"rpm - repository package manager"#);
    let mut about = String::from(env!("CARGO_PKG_DESCRIPTION"));
    about.push_str(" - ");
    about.push_str(env!("CARGO_PKG_REPOSITORY"));
    let mut clap_app = ClapApp::new(title)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(&*about)
        .usage("Press `?` while running the app to see keybindings")
        .before_help(BANNER)
        .after_help("All configs are stored in /etc/rpm")
        .arg(
            Arg::with_name("completions")
                .long("completions")
                .help("Generates completions for your preferred shell")
                .takes_value(true)
                .possible_values(&["bash", "zsh", "fish", "power-shell", "elvish"])
                .value_name("SHELL"),
        )
        // Control spotify from the command line
        .subcommand(cli::playback_subcommand())
        .subcommand(cli::play_subcommand())
        .subcommand(cli::list_subcommand())
        .subcommand(cli::search_subcommand());

    let matches = clap_app.clone().get_matches();

    // Shell completions don't need any spotify work
    if let Some(s) = matches.value_of("completions") {
        let shell = match s {
            "fish" => Shell::Fish,
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "power-shell" => Shell::PowerShell,
            "elvish" => Shell::Elvish,
            _ => return Err(anyhow!("no completions avaible for '{}'", s)),
        };
        clap_app.gen_completions_to("rpm", shell, &mut std::io::stdout());
        return Ok(());
    }

    let config: CliConfig = CliConfig::load_config().await?;
    println!("{:?}", config);

    match matches.subcommand_name().unwrap_or_default() {
        "new" => {}
        _ => {
            let mut app = App::new(config)?;
            let mut terminal = create_terminal(&app)?;
            let mut is_first_render = true;
            while !app.app_should_quit {
                if let Ok(size) = terminal.size() {
                    // Reset the help menu is the terminal was resized
                    if is_first_render || app.windows_settings.size != size {
                        app.windows_settings.help_menu_max_lines = 0;
                        app.windows_settings.help_menu_offset = 0;
                        app.windows_settings.help_menu_page = 0;

                        app.windows_settings.size = size;

                        // Based on the size of the terminal, adjust the search limit.
                        let potential_limit =
                            max((app.windows_settings.size.height as i32) - 13, 0) as u32;
                        let max_limit = min(potential_limit, 50);
                        let large_search_limit =
                            min((f32::from(size.height) / 1.4) as u32, max_limit);
                        let small_search_limit =
                            min((f32::from(size.height) / 2.85) as u32, max_limit / 2);

                        // app.dispatch(IoEvent::UpdateSearchLimits(
                        //     large_search_limit,
                        //     small_search_limit,
                        // ));

                        // Based on the size of the terminal, adjust how many lines are
                        // displayed in the help menu
                        if app.windows_settings.size.height > 8 {
                            app.windows_settings.help_menu_max_lines =
                                app.windows_settings.size.height - 8;
                        } else {
                            app.windows_settings.help_menu_max_lines = 0;
                        }
                    }
                };

                let route = app.router.get_current_route();
                terminal.draw(|f| match route.active_block {
                    ActiveBlock::HelpMenu => {
                        ui::draw_help_page(&app, f);
                    }
                    _ => {
                        ui::draw_main_layout(&app, f);
                    }
                })?;

                if route.active_block == ActiveBlock::Input {
                    terminal.show_cursor()?;
                } else {
                    terminal.hide_cursor()?;
                }

                let cursor_offset = if app.windows_settings.size.height > ui::SMALL_TERMINAL_HEIGHT
                {
                    2
                } else {
                    1
                };

                terminal.backend_mut().execute(MoveTo(
                    cursor_offset + app.input_cursor_position,
                    cursor_offset,
                ))?;

                match app.events.next()? {
                    Event::Input(key) => {
                        match key {
                            Key::Ctrl('c') => app.stop()?,
                            _ => {
                                let current_active_block = app.router.get_current_route().active_block;

                                // To avoid swallowing the global key presses `q` and `-` make a special
                                // case for the input handler
                                if current_active_block == ActiveBlock::Input {
                                    handlers::input_handler(key, &mut app);
                                } else if key == app.config.key_bindings.back {
                                    if app.router.get_current_route().active_block != ActiveBlock::Input {
                                        // Go back through navigation stack when not in search input mode and exit the app if there are no more places to back to

                                        let pop_result = match app.router.pop_navigation_stack() {
                                            Some(ref x) if x.id == RouteId::Search => {
                                                app.router.pop_navigation_stack()
                                            }
                                            Some(x) => Some(x),
                                            None => None,
                                        };
                                        if pop_result.is_none() {
                                            break; // Exit application
                                        }
                                    }
                                } else {
                                    handlers::handle_app(key, &mut app);
                                }
                            }
                        };
                    }
                    Event::Tick => {
                        app.update_on_tick();
                    }
                }

                if is_first_render {
                    // app.dispatch(IoEvent::GetPlaylists);
                    // app.dispatch(IoEvent::GetUser);
                    // app.dispatch(IoEvent::GetCurrentPlayback);
                    // app.help_docs_size =
                    //     ui::help::get_help_docs(&app.user_config.keys).len() as u32;

                    is_first_render = false;
                }
            }
            close_application()?;
        }
    };


    Ok(())

    //       // Work with the cli (not really async)
    //       if let Some(cmd) = matches.subcommand_name() {
    //         // Save, because we checked if the subcommand is present at runtime
    //         let m = matches.subcommand_matches(cmd).unwrap();
    //         let network = Network::new(oauth, spotify, client_config, &app);
    //         println!(
    //           "{}",
    //           cli::handle_matches(m, cmd.to_string(), network, user_config).await?
    //         );
    //       // Launch the UI (async)
    //       } else {
    //         let cloned_app = Arc::clone(&app);
    //         std::thread::spawn(move || {
    //           let mut network = Network::new(oauth, spotify, client_config, &app);
    //           start_tokio(sync_io_rx, &mut network);
    //         });
    //         // The UI must run in the "main" thread
    //         start_ui(user_config, &cloned_app).await?;
    //       }
    //     }
    //     None => println!("\nSpotify auth failed"),
    //   }

    //   Ok(())
    // }

    //

    // async fn start_ui(user_config: UserConfig, app: &Arc<Mutex<App>>) -> Result<()> {
    //   // Terminal initialization
    //   let mut stdout = stdout();
    //   execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    //   enable_raw_mode()?;

    //   let mut backend = CrosstermBackend::new(stdout);

    //   if user_config.behavior.set_window_title {
    //     backend.execute(SetTitle("spt - Spotify TUI"))?;
    //   }

    //   let mut terminal = Terminal::new(backend)?;
    //   terminal.hide_cursor()?;

    //   let events = event::Events::new(user_config.behavior.tick_rate_milliseconds);

    //   // play music on, if not send them to the device selection view

    //   let mut is_first_render = true;

    //   loop {
    //     let mut app = app.lock().await;
    //     // Get the size of the screen on each loop to account for resize event
    //     if let Ok(size) = terminal.backend().size() {
    //       // Reset the help menu is the terminal was resized
    //       if is_first_render || app.size != size {
    //         app.help_menu_max_lines = 0;
    //         app.help_menu_offset = 0;
    //         app.help_menu_page = 0;

    //         app.size = size;

    //         // Based on the size of the terminal, adjust the search limit.
    //         let potential_limit = max((app.size.height as i32) - 13, 0) as u32;
    //         let max_limit = min(potential_limit, 50);
    //         let large_search_limit = min((f32::from(size.height) / 1.4) as u32, max_limit);
    //         let small_search_limit = min((f32::from(size.height) / 2.85) as u32, max_limit / 2);

    //         app.dispatch(IoEvent::UpdateSearchLimits(
    //           large_search_limit,
    //           small_search_limit,
    //         ));

    //         // Based on the size of the terminal, adjust how many lines are
    //         // displayed in the help menu
    //         if app.size.height > 8 {
    //           app.help_menu_max_lines = (app.size.height as u32) - 8;
    //         } else {
    //           app.help_menu_max_lines = 0;
    //         }
    //       }
    //     };

    //     let current_route = app.get_current_route();
    //     terminal.draw(|mut f| match current_route.active_block {
    //       ActiveBlock::HelpMenu => {
    //         ui::draw_help_menu(&mut f, &app);
    //       }
    //       ActiveBlock::Error => {
    //         ui::draw_error_screen(&mut f, &app);
    //       }
    //       ActiveBlock::SelectDevice => {
    //         ui::draw_device_list(&mut f, &app);
    //       }
    //       ActiveBlock::Analysis => {
    //         ui::audio_analysis::draw(&mut f, &app);
    //       }
    //       ActiveBlock::BasicView => {
    //         ui::draw_basic_view(&mut f, &app);
    //       }
    //       _ => {
    //         ui::draw_main_layout(&mut f, &app);
    //       }
    //     })?;

    //     if current_route.active_block == ActiveBlock::Input {
    //       terminal.show_cursor()?;
    //     } else {
    //       terminal.hide_cursor()?;
    //     }

    //     let cursor_offset = if app.size.height > ui::util::SMALL_TERMINAL_HEIGHT {
    //       2
    //     } else {
    //       1
    //     };

    //     // Put the cursor back inside the input box
    //     terminal.backend_mut().execute(MoveTo(
    //       cursor_offset + app.input_cursor_position,
    //       cursor_offset,
    //     ))?;

    //     // Handle authentication refresh
    //     if SystemTime::now() > app.spotify_token_expiry {
    //       app.dispatch(IoEvent::RefreshAuthentication);
    //     }

    //     match events.next()? {
    //       event::Event::Input(key) => {
    //         if key == Key::Ctrl('c') {
    //           break;
    //         }

    //         let current_active_block = app.get_current_route().active_block;

    //         // To avoid swallowing the global key presses `q` and `-` make a special
    //         // case for the input handler
    //         if current_active_block == ActiveBlock::Input {
    //           handlers::input_handler(key, &mut app);
    //         } else if key == app.user_config.keys.back {
    //           if app.get_current_route().active_block != ActiveBlock::Input {
    //             // Go back through navigation stack when not in search input mode and exit the app if there are no more places to back to

    //             let pop_result = match app.pop_navigation_stack() {
    //               Some(ref x) if x.id == RouteId::Search => app.pop_navigation_stack(),
    //               Some(x) => Some(x),
    //               None => None,
    //             };
    //             if pop_result.is_none() {
    //               break; // Exit application
    //             }
    //           }
    //         } else {
    //           handlers::handle_app(key, &mut app);
    //         }
    //       }
    //       event::Event::Tick => {
    //         app.update_on_tick();
    //       }
    //     }

    //     // Delay spotify request until first render, will have the effect of improving
    //     // startup speed
    //     if is_first_render {
    //       app.dispatch(IoEvent::GetPlaylists);
    //       app.dispatch(IoEvent::GetUser);
    //       app.dispatch(IoEvent::GetCurrentPlayback);
    //       app.help_docs_size = ui::help::get_help_docs(&app.user_config.keys).len() as u32;

    //       is_first_render = false;
    //     }
    //   }

    // terminal.show_cursor()?;
    // close_application()?;
}
