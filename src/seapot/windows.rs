use super::palette;
use crossterm::event::KeyCode;
use rspotify::spotify::{client::Spotify, model::track::SavedTrack};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::Style,
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
    cursor: usize,
}

impl LikedSongs {
    pub fn new() -> LikedSongs {
        LikedSongs {
            song_list: vec![],
            page_number: 0,
            cursor: 0,
        }
    }
    pub fn get_next_page(&mut self, rspotify_client: &Spotify) {
        let mut page = rspotify_client
            .current_user_saved_tracks(50, self.page_number * 50)
            .expect("Can not get next page of saved tracks");
        self.song_list.append(&mut page.items);
        self.page_number += 1;
    }

    pub fn key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor != 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.cursor < self.song_list.len() {
                    self.cursor += 1;
                }
            }
            _ => (),
        }
    }

    pub fn draw(&self, f: &mut Frame<Backend>, area: Rect) {
        let mut text = vec![];
        for song in self.song_list.iter() {
            text.push(format!("{}", song.track.name));
        }
        SelectableList::default()
            .items(&text)
            .block(Block::default().title("LikedSongs").borders(Borders::ALL))
            .select(Some(self.cursor))
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
