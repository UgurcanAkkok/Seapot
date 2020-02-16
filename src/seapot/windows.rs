use super::palette;
use rspotify::spotify::{client::Spotify, model::track::SavedTrack};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget},
    Frame,
};

pub type Backend = CrosstermBackend<Stdout>;
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}
impl MessageLevel {
    pub fn get_style(&self) -> Style {
        match self {
            MessageLevel::Info => Style::default().fg(palette::GREY).bg(palette::BLACK),
            MessageLevel::Warning => Style::default().fg(palette::WHITE).bg(palette::BLACK),
            MessageLevel::Error => Style::default().fg(Color::Red).bg(palette::BLACK),
        }
    }
}

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
        LikedSongs { song_list: vec![] }
    }
    pub fn synchronize(&mut self, rspotify_client: &Spotify) {
        let page = rspotify_client
            .current_user_saved_tracks(50, 0)
            .expect("Can not get saved tracks");
        self.song_list = page.items;
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

pub struct Message {
    text: String,
    level: MessageLevel,
}
impl Message {
    pub fn new() -> Message {
        Message {
            text: String::default(),
            level: MessageLevel::Info,
        }
    }

    pub fn set_message(&mut self, msg: String, level: MessageLevel) {
        self.text = msg;
        self.level = level;
    }

    pub fn draw(&self, f: &mut Frame<Backend>, area: Rect) {
        let message = Text::styled(&self.text, self.level.get_style());
        let text = vec![message];
        Paragraph::new(text.iter())
            .block(Block::default())
            .style(Style::default().bg(palette::BLACK))
            .wrap(true)
            .render(f, area);
    }
}

pub enum Page {
    Welcome,
    HomePage,
}
