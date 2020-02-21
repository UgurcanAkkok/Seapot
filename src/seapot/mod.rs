use crossterm::event::KeyCode;
use std::io::{self};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};

pub mod musicplayer;
mod palette;
mod windowmanager;
use windowmanager::*;
mod windows;
use windows::*;

use rspotify::spotify::{
    client::Spotify,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth},
    util::get_token,
};

const SCOPES: [&'static str; 1] = ["user-library-read"];

// Visual part of the program, i.e. pages and
// terminal backend. Drawing is done by this struct
pub struct Seapot {
    terminal: Terminal<Backend>,
    page: Page,
    spotify: Spotify,
    wm: WindowManager,
    redraw: bool,
}

impl Seapot {
    pub fn new() -> Seapot {
        let output = io::stdout();
        let backend = CrosstermBackend::new(output);
        let terminal = Terminal::new(backend).unwrap();
        let page = Page::Welcome;
        let wm = WindowManager::new();

        let mut oauth = SpotifyOAuth::default()
            .client_id("897b20ba14694706abc9f6dce9d86609")
            .client_secret("9245bbebbd424d3eae3b271df0b4ee29")
            .redirect_uri("http://localhost:8888/callback")
            .scope(&SCOPES.join(" "))
            .build();

        let spotify;
        match get_token(&mut oauth) {
            Some(token) => {
                let creds = SpotifyClientCredentials::default()
                    .token_info(token)
                    .build();
                spotify = Spotify::default().client_credentials_manager(creds).build();
            }
            None => panic!("Can not get Token for SpotifyOAuth"),
        }

        Seapot {
            terminal,
            page,
            spotify,
            wm,
            redraw: true,
        }
    }

    pub fn homepage(&mut self) {
        match self.page {
            Page::HomePage => (),
            _ => {
                self.page = Page::HomePage;
                self.wm.focuse(Window::LikedSongs);
            }
        }
        self.redraw = true;
    }

    pub fn get_liked_songs_more(&mut self) {
        self.wm.liked_songs.get_next_page(&self.spotify);
    }

    pub fn draw(&mut self) {
        if self.redraw == false {
            return;
        }
        self.terminal.hide_cursor().unwrap();
        let page = &self.page;
        let wm = &self.wm;
        self.terminal
            .draw(|mut f| match page {
                Page::Welcome => wm.welcome.draw(&mut f),
                Page::HomePage => {
                    let chunks = Layout::default()
                        .constraints([Constraint::Min(5)].as_ref())
                        .split(f.size());
                    wm.liked_songs.draw(&mut f, chunks[0]);
                }
            })
            .unwrap();
    }

    pub fn process_key(&mut self, key: KeyCode) {
        self.wm.process_key(key);
    }
}
