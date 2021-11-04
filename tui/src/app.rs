use crate::router::Router;
use anyhow::Result;
use tui::layout::Rect;

use crate::{config::Config, event::Events};
use manager::Engine;

const TITLE: &str = "repo version-manager";

#[derive(Clone, Debug)]
pub struct HelpSettings {
    pub help_menu_offset: u32,
    pub help_menu_max_lines: u32,
    pub help_docs_size: u32,
    pub help_menu_page: u32,
}

impl Default for HelpSettings {
    fn default() -> HelpSettings {
        HelpSettings {
            help_menu_offset: 0,
            help_menu_max_lines: 0,
            help_docs_size: 0,
            help_menu_page: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowSettings {
    pub size_width: u16,
    pub home_scroll: u16,
    pub window_height: u16,
    pub help_menu_max_lines: u16,
    pub help_menu_offset: u16,
    pub help_menu_page: u16,
    pub size: Rect,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            size_width: 0,
            home_scroll: 0,
            window_height: 6,
            help_menu_max_lines: 0,
            help_menu_offset: 0,
            help_menu_page: 0,
            size: Rect::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct App {
    pub title: String,
    pub config: Config,
    pub app_should_quit: bool,
    pub engine: Engine,
    pub events: Events,
    pub help_settings: HelpSettings,
    pub router: Router,
    pub windows_settings: WindowSettings,
    pub is_loading: bool,
    pub input: Vec<char>,
    pub input_idx: usize,
    pub input_cursor_position: u16,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let package_manager = config.package_manager.clone();
        let engine = Engine::new(
            package_manager.package_list_file_path,
            package_manager.history_path,
        );
        let events = Events::new(config.behavior.tick_rate_milliseconds);

        Ok(App {
            title: String::from(TITLE),
            config,
            events,
            app_should_quit: false,
            engine,
            help_settings: HelpSettings::default(),
            router: Router::new(),
            windows_settings: WindowSettings::default(),
            input: vec![],
            input_cursor_position: 0,
            input_idx: 0,
            is_loading: false,
        })
    }

    pub fn calculate_help_menu_offset(&mut self) {
        let old_offset = self.help_settings.help_menu_offset;

        if self.help_settings.help_menu_max_lines < self.help_settings.help_docs_size {
            self.help_settings.help_menu_offset =
                self.help_settings.help_menu_page * self.help_settings.help_menu_max_lines;
        }
        if self.help_settings.help_menu_offset > self.help_settings.help_docs_size {
            self.help_settings.help_menu_offset = old_offset;
            self.help_settings.help_menu_page -= 1;
        }
    }

    pub fn stop(&mut self) -> Result<()> {
        self.app_should_quit = true;

        Ok(())
    }
    pub fn update_on_tick(&mut self) {}
}
