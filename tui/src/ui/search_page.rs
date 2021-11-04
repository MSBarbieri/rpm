use crate::App;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout,Rect},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub fn draw_search_results(app: &App,frame: &mut Frame<'_, CrosstermBackend<Stdout>>, layout_chunk: Rect)
{
//   let chunks = Layout::default()
//     .direction(Direction::Vertical)
//     .constraints(
//       [
//         Constraint::Percentage(35),
//         Constraint::Percentage(35),
//         Constraint::Percentage(25),
//       ]
//       .as_ref(),
//     )
//     .split(layout_chunk);

//   {
//     let song_artist_block = Layout::default()
//       .direction(Direction::Horizontal)
//       .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//       .split(chunks[0]);

//     let currently_playing_id = app
//       .current_playback_context
//       .clone()
//       .and_then(|context| {
//         context.item.and_then(|item| match item {
//           PlayingItem::Track(track) => track.id,
//           PlayingItem::Episode(episode) => Some(episode.id),
//         })
//       })
//       .unwrap_or_else(|| "".to_string());

//     let songs = match &app.search_results.tracks {
//       Some(tracks) => tracks
//         .items
//         .iter()
//         .map(|item| {
//           let mut song_name = "".to_string();
//           let id = item.clone().id.unwrap_or_else(|| "".to_string());
//           if currently_playing_id == id {
//             song_name += "▶ "
//           }
//           if app.liked_song_ids_set.contains(&id) {
//             song_name += &app.user_config.padded_liked_icon();
//           }

//           song_name += &item.name;
//           song_name += &format!(" - {}", &create_artist_string(&item.artists));
//           song_name
//         })
//         .collect(),
//       None => vec![],
//     };

//     draw_selectable_list(
//       frame,
//       app,
//       song_artist_block[0],
//       "Songs",
//       &songs,
//       get_search_results_highlight_state(app, SearchResultBlock::SongSearch),
//       app.search_results.selected_tracks_index,
//     );

//     let artists = match &app.search_results.artists {
//       Some(artists) => artists
//         .items
//         .iter()
//         .map(|item| {
//           let mut artist = String::new();
//           if app.followed_artist_ids_set.contains(&item.id.to_owned()) {
//             artist.push_str(&app.user_config.padded_liked_icon());
//           }
//           artist.push_str(&item.name.to_owned());
//           artist
//         })
//         .collect(),
//       None => vec![],
//     };

//     draw_selectable_list(
//       frame,
//       app,
//       song_artist_block[1],
//       "Artists",
//       &artists,
//       get_search_results_highlight_state(app, SearchResultBlock::ArtistSearch),
//       app.search_results.selected_artists_index,
//     );
//   }

//   {
//     let albums_playlist_block = Layout::default()
//       .direction(Direction::Horizontal)
//       .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//       .split(chunks[1]);

//     let albums = match &app.search_results.albums {
//       Some(albums) => albums
//         .items
//         .iter()
//         .map(|item| {
//           let mut album_artist = String::new();
//           if let Some(album_id) = &item.id {
//             if app.saved_album_ids_set.contains(&album_id.to_owned()) {
//               album_artist.push_str(&app.user_config.padded_liked_icon());
//             }
//           }
//           album_artist.push_str(&format!(
//             "{} - {} ({})",
//             item.name.to_owned(),
//             create_artist_string(&item.artists),
//             item.album_type.as_deref().unwrap_or("unknown")
//           ));
//           album_artist
//         })
//         .collect(),
//       None => vec![],
//     };

//     draw_selectable_list(
//       frame,
//       app,
//       albums_playlist_block[0],
//       "Albums",
//       &albums,
//       get_search_results_highlight_state(app, SearchResultBlock::AlbumSearch),
//       app.search_results.selected_album_index,
//     );

//     let playlists = match &app.search_results.playlists {
//       Some(playlists) => playlists
//         .items
//         .iter()
//         .map(|item| item.name.to_owned())
//         .collect(),
//       None => vec![],
//     };
//     draw_selectable_list(
//       frame,
//       app,
//       albums_playlist_block[1],
//       "Playlists",
//       &playlists,
//       get_search_results_highlight_state(app, SearchResultBlock::PlaylistSearch),
//       app.search_results.selected_playlists_index,
//     );
//   }

//   {
//     let podcasts_block = Layout::default()
//       .direction(Direction::Horizontal)
//       .constraints([Constraint::Percentage(100)].as_ref())
//       .split(chunks[2]);

//     let podcasts = match &app.search_results.shows {
//       Some(podcasts) => podcasts
//         .items
//         .iter()
//         .map(|item| {
//           let mut show_name = String::new();
//           if app.saved_show_ids_set.contains(&item.id.to_owned()) {
//             show_name.push_str(&app.user_config.padded_liked_icon());
//           }
//           show_name.push_str(&format!("{:} - {}", item.name, item.publisher));
//           show_name
//         })
//         .collect(),
//       None => vec![],
//     };
//     draw_selectable_list(
//       frame,
//       app,
//       podcasts_block[0],
//       "Podcasts",
//       &podcasts,
//       get_search_results_highlight_state(app, SearchResultBlock::ShowSearch),
//       app.search_results.selected_shows_index,
//     );
//   }
}
