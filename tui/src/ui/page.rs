use crate::ui::draw_input_and_help_box;
use crate::ui::draw_search_results;
use crate::{
    app::App,
    banner::BANNER,
    router::{ActiveBlock, RouteId},
};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap,
    },
    Frame,
};

use super::draw_home_page;
pub const BASIC_VIEW_HEIGHT: u16 = 6;
pub const SMALL_TERMINAL_WIDTH: u16 = 150;
pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

fn get_main_layout_margin(app: &App) -> u16 {
    if app.windows_settings.window_height > SMALL_TERMINAL_HEIGHT {
        1
    } else {
        0
    }
}

pub fn draw_main_layout(app: &App, frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
    let margin = get_main_layout_margin(app);
    // Responsive layout: new one kicks in at width 150 or higher
    if app.windows_settings.size_width >= SMALL_TERMINAL_WIDTH
        && !app.config.behavior.enforce_wide_search_bar
    {
        let parent_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(6)].as_ref())
            .margin(margin)
            .split(frame.size());

        // Nested main block with potential routes
        draw_routes(app, frame, parent_layout[0]);

        // Currently playing
        // draw_playbar(app,frame, parent_layout[1]);
    } else {
        let parent_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(6),
                ]
                .as_ref(),
            )
            .margin(margin)
            .split(frame.size());

        // Search input and help
        draw_input_and_help_box(app, frame, parent_layout[0]);

        // Nested main block with potential routes
        draw_routes(app, frame, parent_layout[1]);

        // Currently playing
        // draw_playbar(app,frame, parent_layout[2]);
    }

    // Possibly draw confirm dialog
    //   draw_dialog(app,frame);
}

fn draw_routes(app: &App, f: &mut Frame<'_, CrosstermBackend<Stdout>>, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    // draw_user_block(f, app, chunks[0]);

    let current_route = app.router.get_current_route();

    match current_route.id {
        RouteId::Home => {
            draw_home_page(app, f, chunks[1]);
        }
        RouteId::Error => {} // This is handled as a "full screen" route in main.rs
        RouteId::Analysis => {} // This is handled as a "full screen" route in main.rs
        RouteId::Search => {
            draw_search_results(app, f, chunks[1]);
        }
    };
}
