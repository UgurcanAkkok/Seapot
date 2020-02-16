use std::io::{self};
use tui::{backend::CrosstermBackend, Terminal};

pub mod musicplayer;
mod palette;
mod windows;
use windows::*;

use rspotify::spotify::{
    util::get_token,
    client::Spotify,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth},
};

const SCOPES : [&'static str; 1] = [
    "user-library-read",
];
// Visual part of the program, i.e. pages and 
// terminal backend. Drawing is done by this struct
pub struct Seapot {
    terminal: Terminal<Backend>,
    page: Vec<Window>,
    spotify: Spotify,
}

impl Seapot {
    pub fn new() -> Seapot {
        let output = io::stdout();
        let backend = CrosstermBackend::new(output);
        let terminal = Terminal::new(backend).unwrap();
        let page = vec![Window::Welcome(windows::Welcome::new())];

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
                spotify = Spotify::default()
                    .client_credentials_manager(creds)
                    .build();
            },
            None => panic!("Can not get Token for SpotifyOAuth"),
        }

        Seapot { terminal, page, spotify }
    }

    pub fn page_liked_songs(&mut self){
        let mut page = LikedSongs::new();
        page.synchronize(&self.spotify);
        self.page = vec![Window::LikedSongs(page)];
    }

    pub fn draw(&mut self) {
        self.terminal.hide_cursor().unwrap();
        let page = &self.page;
        self.terminal
            .draw(|mut f| {
                for w in page.iter() {
                    match w {
                        Window::Welcome(win) => win.draw(&mut f),
                        Window::LikedSongs(win) => win.draw(&mut f),
                    }
                }
            })
            .unwrap();
    }
}
