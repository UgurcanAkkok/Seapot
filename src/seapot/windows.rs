use super::palette;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders, Paragraph, Text, Widget, SelectableList},
    Frame,
};
use rspotify::spotify::{
    util::get_token,
    client::Spotify,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth},
    model::track::SavedTrack,
};

pub type Backend = CrosstermBackend<Stdout>;
pub struct Welcome {}
impl Welcome {
    pub fn draw(&self, f: &mut Frame<Backend>) {
        let text = vec![Text::raw("Welcome to Seapot!")];
        Paragraph::new(text.iter())
            .block(Block::default().title("Welcome").borders(Borders::ALL))
            .style(Style::default().fg(palette::GREEN).bg(palette::BLACK))
            .alignment(Alignment::Center)
            .render(f, f.size())
    }

    pub fn new() -> Welcome {
        Welcome {}
    }
}

pub struct LikedSongs {
    song_list: Vec<SavedTrack>,
}

impl LikedSongs {
    pub fn new() -> LikedSongs {
        LikedSongs {
            song_list: vec![],
        }
    }
    pub fn synchronize(&mut self, rspotify_client: &Spotify){
        let page = rspotify_client
            .current_user_saved_tracks(50,0)
            .expect("Can not get saved tracks");
        self.song_list = page.items;
    }

    pub fn draw(&self, f: &mut Frame<Backend>){
        let mut text = vec![];
        for song in self.song_list.iter() {
            text.push(format!("{}", song.track.name));
        }
        SelectableList::default()
            .items(&text)
            .block(Block::default().title("LikedSongs").borders(Borders::ALL))
            .select(Some(1))
            .style(Style::default().fg(palette::WHITE).bg(palette::BLACK))
            .highlight_style(Style::default().fg(palette::GREEN).bg(palette::BLACK))
            .highlight_symbol("⚫")
            .render(f, f.size());
    }
}
pub enum Window {
    Welcome(Welcome),
    LikedSongs(LikedSongs),
}
