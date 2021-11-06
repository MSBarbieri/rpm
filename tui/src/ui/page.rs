use std::io::Stdout;
use tui::widgets::{Borders, Paragraph};
use tui::{backend::CrosstermBackend, Frame};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::Block,
};

use crate::app::App;

pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

fn get_main_layout_margin(app: &App) -> u16 {
    if app.windows_settings.window_height <= SMALL_TERMINAL_HEIGHT {
        1
    } else {
        0
    }
}

pub fn home_page(app: &App, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let margin = get_main_layout_margin(app);
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .margin(margin)
        .split(frame.size());

    let style = Style::default().fg(Color::Gray);

    let name_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(95), Constraint::Min(5)])
        .split(parent_layout[0]);
    let name_block = Block::default()
        .title(Span::styled("who am i", style))
        .borders(Borders::ALL)
        .border_style(style);

    let whoami = Paragraph::new("foo")
        .block(name_block)
        .style(style);

    frame.render_widget(whoami, name_layout[0]);
    super::help_page::draw_help_box(app, frame, name_layout[1]);

    let data_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(parent_layout[1]);

    let info_block = Block::default()
        .title(Span::styled("info", style))
        .borders(Borders::ALL)
        .border_style(style);

    let content_block = Block::default()
        .title(Span::styled("content", style))
        .borders(Borders::ALL)
        .border_style(style);

    let info = Paragraph::new("bar")
        .block(info_block)
        .style(style);
    let content = Paragraph::new("baz")
        .block(content_block)
        .style(style);
    frame.render_widget(info, data_layout[0]);
    frame.render_widget(content, data_layout[1]);
}

pub fn help_page(app: &App, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    super::help_page::draw_help_page(app, frame);
}
