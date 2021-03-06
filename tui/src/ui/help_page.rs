use crate::App;
use std::io::Stdout;
use tui::layout::Rect;
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub fn draw_help_box(
    app: &App,
    frame: &mut Frame<'_, CrosstermBackend<Stdout>>,
    layout_chunk: Rect,
) {
    // Check for the width and change the contraints accordingly

    let show_loading = app.is_loading;
    let help_block_text = if show_loading {
        (Color::Yellow, "Loading...")
    } else {
        (Color::Gray, "Type ?")
    };

    let block = Block::default()
        .title(Span::styled("Help", Style::default().fg(help_block_text.0)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(help_block_text.0));

    let lines = Text::from(help_block_text.1);
    let help = Paragraph::new(lines)
        .block(block)
        .style(Style::default().fg(help_block_text.0));
    frame.render_widget(help, layout_chunk);
}

pub fn draw_help_page(_: &App, frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(frame.size());

    let format_row =
        |r: Vec<String>| -> Vec<String> { vec![format!("{:50}{:40}{:20}", r[0], r[1], r[2])] };

    let help_menu_style = Style::default().fg(Color::Reset);
    let header = ["Description", "Event", "Context"];
    let header = format_row(header.iter().map(|s| s.to_string()).collect());

    let help_docs = get_help_docs();
    let help_docs = help_docs
        .into_iter()
        .map(format_row)
        .collect::<Vec<Vec<String>>>();
    let help_docs = &help_docs[0 as usize..];

    let rows = help_docs
        .iter()
        .map(|item| Row::new(item.clone()).style(help_menu_style));

    let help_menu = Table::new(rows)
        .header(Row::new(header))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(help_menu_style)
                .title(Span::styled(
                    "Help (press <Esc> to go back)",
                    help_menu_style,
                ))
                .border_style(help_menu_style),
        )
        .style(help_menu_style)
        .widths(&[Constraint::Percentage(100)]);
    frame.render_widget(help_menu, chunks[0]);
}

pub fn get_help_docs() -> Vec<Vec<String>> {
    vec![
        vec![
            String::from("Scroll down to next result page"),
            String::from("j"),
            String::from("Pagination"),
        ],
        // vec![
        //     String::from("Scroll up to previous result page"),
        //     key_bindings.previous_page.to_string(),
        //     String::from("Pagination"),
        // ],
        // vec![
        //     String::from("Seek backwards 5 seconds"),
        //     key_bindings.seek_backwards.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Seek forwards 5 seconds"),
        //     key_bindings.seek_forwards.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Toggle shuffle"),
        //     key_bindings.shuffle.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Copy url to currently playing song/episode"),
        //     key_bindings.copy_song_url.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Copy url to currently playing album/show"),
        //     key_bindings.copy_album_url.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Cycle repeat mode"),
        //     key_bindings.repeat.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Move selection left"),
        //     String::from("h | <Left Arrow Key> | <Ctrl+b>"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Move selection down"),
        //     String::from("j | <Down Arrow Key> | <Ctrl+n>"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Move selection up"),
        //     String::from("k | <Up Arrow Key> | <Ctrl+p>"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Move selection right"),
        //     String::from("l | <Right Arrow Key> | <Ctrl+f>"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Move selection to top of list"),
        //     String::from("H"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Move selection to middle of list"),
        //     String::from("M"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Move selection to bottom of list"),
        //     String::from("L"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Enter input for search"),
        //     key_bindings.search.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Enter active mode"),
        //     String::from("<Enter>"),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Go to audio analysis screen"),
        //     key_bindings.audio_analysis.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Go to playbar only screen (basic view)"),
        //     key_bindings.basic_view.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Go back or exit when nowhere left to back to"),
        //     key_bindings.back.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Select device to play music on"),
        //     key_bindings.manage_devices.to_string(),
        //     String::from("General"),
        // ],
        // vec![
        //     String::from("Enter hover mode"),
        //     String::from("<Esc>"),
        //     String::from("Selected block"),
        // ],
        // vec![
        //     String::from("Save track in list or table"),
        //     String::from("s"),
        //     String::from("Selected block"),
        // ],
        // vec![
        //     String::from("Start playback or enter album/artist/playlist"),
        //     key_bindings.submit.to_string(),
        //     String::from("Selected block"),
        // ],
        // vec![
        //     String::from("Play recommendations for song/artist"),
        //     String::from("r"),
        //     String::from("Selected block"),
        // ],
        // vec![
        //     String::from("Play all tracks for artist"),
        //     String::from("e"),
        //     String::from("Library -> Artists"),
        // ],
        // vec![
        //     String::from("Search with input text"),
        //     String::from("<Enter>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Move cursor one space left"),
        //     String::from("<Left Arrow Key>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Move cursor one space right"),
        //     String::from("<Right Arrow Key>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Delete entire input"),
        //     String::from("<Ctrl+l>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Delete text from cursor to start of input"),
        //     String::from("<Ctrl+u>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Delete text from cursor to end of input"),
        //     String::from("<Ctrl+k>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Delete previous word"),
        //     String::from("<Ctrl+w>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Jump to start of input"),
        //     String::from("<Ctrl+a>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Jump to end of input"),
        //     String::from("<Ctrl+e>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Escape from the input back to hovered block"),
        //     String::from("<Esc>"),
        //     String::from("Search input"),
        // ],
        // vec![
        //     String::from("Delete saved album"),
        //     String::from("D"),
        //     String::from("Library -> Albums"),
        // ],
        // vec![
        //     String::from("Delete saved playlist"),
        //     String::from("D"),
        //     String::from("Playlist"),
        // ],
        // vec![
        //     String::from("Follow an artist/playlist"),
        //     String::from("w"),
        //     String::from("Search result"),
        // ],
        // vec![
        //     String::from("Save (like) album to library"),
        //     String::from("w"),
        //     String::from("Search result"),
        // ],
        // vec![
        //     String::from("Play random song in playlist"),
        //     String::from("S"),
        //     String::from("Selected Playlist"),
        // ],
        // vec![
        //     String::from("Toggle sort order of podcast episodes"),
        //     String::from("S"),
        //     String::from("Selected Show"),
        // ],
        // vec![
        //     String::from("Add track to queue"),
        //     key_bindings.add_item_to_queue.to_string(),
        //     String::from("Hovered over track"),
        // ],
    ]
}
