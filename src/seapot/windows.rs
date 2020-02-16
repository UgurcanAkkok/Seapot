use super::palette;
use rspotify::spotify::{client::Spotify, model::track::SavedTrack};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{
        Style
    },
    widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget},
    Frame,
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
    page_number: u32,

}
impl LikedSongs {
    pub fn new() -> LikedSongs {
        LikedSongs { song_list: vec![], page_number: 0 }
    }
    pub fn get_next_page(&mut self, rspotify_client: &Spotify) {
        let page = rspotify_client
            .current_user_saved_tracks(50, self.page_number * 50)
            .expect("Can not get next page of saved tracks");
        self.song_list = page.items;
        self.page_number += 1;
    }

    pub fn draw(&self, f: &mut Frame<Backend>, area: Rect) {
        let mut text = vec![];
        for song in self.song_list.iter() {
            text.push(format!("{}", song.track.name));
        }
        SelectableList::default()
            .items(&text)
            .block(Block::default().title("LikedSongs").borders(Borders::ALL))
            .select(Some(0))
            .style(Style::default().fg(palette::WHITE).bg(palette::BLACK))
            .highlight_style(Style::default().fg(palette::GREEN).bg(palette::BLACK))
            .highlight_symbol("⚫")
            .render(f, area);
    }
}

pub enum Page {
    Welcome,
    HomePage,
}
