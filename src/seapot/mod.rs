use std::io::{self};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};
//use crossterm::{
//    event::{Event, KeyCode, KeyEvent},
//};

pub mod musicplayer;
mod palette;
mod windows;
use windows::*;

use rspotify::spotify::{
    client::Spotify,
    oauth2::{SpotifyClientCredentials, SpotifyOAuth},
    util::get_token,
};

const SCOPES: [&'static str; 1] = ["user-library-read"];

struct WindowManager {
    pub liked_songs: LikedSongs,
    pub message: Message,
    pub welcome: Welcome,
}
impl WindowManager {
    pub fn new() -> WindowManager {
        WindowManager {
            liked_songs: LikedSongs::new(),
            message: Message::new(),
            welcome: Welcome::new(),
        }
    }
}
// Visual part of the program, i.e. pages and
// terminal backend. Drawing is done by this struct
pub struct Seapot {
    terminal: Terminal<Backend>,
    page: Page,
    spotify: Spotify,
    wm: WindowManager,
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
        }
    }

    pub fn homepage(&mut self) {
        match self.page {
            Page::HomePage => (),
            _ => {
                self.page = Page::HomePage;
            }
        }
    }

    pub fn synchronize_all(&mut self) {
        self.wm.message.set_message("Syncing the account..".to_string(), MessageLevel::Info);
        self.wm.liked_songs.synchronize(&self.spotify)
    }

    pub fn draw(&mut self) {
        self.terminal.hide_cursor().unwrap();
        let page = &self.page;
        let wm = &self.wm;
        self.terminal
            .draw(|mut f| match page {
                Page::Welcome => wm.welcome.draw(&mut f),
                Page::HomePage => {
                    let chunks = Layout::default()
                        .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
                        .split(f.size());
                    wm.liked_songs.draw(&mut f, chunks[0]);
                    wm.message.draw(&mut f, chunks[1]);
                }
            })
            .unwrap();
    }

    //pub fn process_event(&self, key_event: KeyEvent){
    //    match key_event {

    //}
}
