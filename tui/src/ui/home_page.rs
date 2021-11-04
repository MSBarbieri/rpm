use crate::{router::ActiveBlock, ui::get_color, App, BANNER};
use std::{include_str, io::Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw_home_page(
    app: &App,
    frame: &mut Frame<'_, CrosstermBackend<Stdout>>,
    layout_chunk: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
        .margin(2)
        .split(layout_chunk);

    let current_route = app.router.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );

    let welcome = Block::default()
        .title(Span::styled(
            "Welcome!",
            get_color(highlight_state, &app.config.theme),
        ))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, &app.config.theme));
    frame.render_widget(welcome, layout_chunk);

    let changelog = include_str!("../../CHANGELOG.md").to_string();

    // If debug mode show the "Unreleased" header. Otherwise it is a release so there should be no
    // unreleased features
    let clean_changelog = if cfg!(debug_assertions) {
        changelog
    } else {
        changelog.replace("\n## [Unreleased]\n", "")
    };

    // Banner text with correct styling
    let mut top_text = Text::from(BANNER);
    top_text.patch_style(Style::default().fg(app.config.theme.banner));

    let bottom_text_raw = format!(
    "{}{}",
    "\nPlease report any bugs or missing features to https://github.com/Rigellute/spotify-tui\n\n",
    clean_changelog
  );
    let bottom_text = Text::from(bottom_text_raw.as_str());

    // Contains the banner
    let top_text = Paragraph::new(top_text)
        .style(Style::default().fg(app.config.theme.text))
        .block(Block::default());
    frame.render_widget(top_text, chunks[0]);

    // CHANGELOG
    let bottom_text = Paragraph::new(bottom_text)
        .style(Style::default().fg(app.config.theme.text))
        .block(Block::default())
        .wrap(Wrap { trim: false })
        .scroll((app.windows_settings.home_scroll, 0));
    frame.render_widget(bottom_text, chunks[1]);
}
