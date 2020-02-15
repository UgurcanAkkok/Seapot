use std::io::{self};

use tui::{backend::CrosstermBackend, Terminal};

use librespot::{
    core::{
        authentication::Credentials,
        config::SessionConfig,
        session::Session,
        spotify_id::{SpotifyId, },
    },
    playback::{
        audio_backend,
        config::PlayerConfig,
        player::{Player, },
    },
};
use tokio_core::reactor::Core;

mod palette;
mod windows;
use windows::*;

pub struct Seapot {
    terminal: Terminal<Backend>,
    page: Vec<Window>,
}

pub struct MusicPlayer {
    player: Player,
    session: Session,
    core: Core,
}

impl MusicPlayer {
    pub fn new(user: String, pass: String) -> MusicPlayer {
        let mut core = Core::new().unwrap();
        let core_handle = core.handle();
        let session_config = SessionConfig::default();
        let player_config = PlayerConfig::default();
        let creds = Credentials::with_password(user, pass);
        let backend = audio_backend::find(None).unwrap();
        let session = core
            .run(Session::connect(session_config, creds, None, core_handle))
            .unwrap();

        let (player, _) = Player::new(player_config, session.clone(), None, move || {
            (backend)(None)
        });
        MusicPlayer {
            player,
            session,
            core,
        }
    }
    pub fn play_track(&mut self, id: &str) {
        let track = SpotifyId::from_base62(id)
            .or_else(|e| {
                eprintln!("Wrong id for track. Err: {:?}", e);
                Err(e)
            })
            .unwrap();
        self.core
            .run(self.player.load(track, true, 0))
            .expect("Can not load and play the song");
    }
}
impl Seapot {
    pub fn new() -> Seapot {
        let output = io::stdout();
        let backend = CrosstermBackend::new(output);
        let terminal = Terminal::new(backend).unwrap();
        let page = vec![Window::Welcome(windows::Welcome::new())];

        Seapot { terminal, page }
    }

    pub fn draw(&mut self) {
        self.terminal.hide_cursor().unwrap();
        let page = &self.page;
        self.terminal
            .draw(|mut f| {
                for w in page.iter() {
                    match w {
                        Window::Welcome(win) => win.draw(&mut f),
                    }
                }
            })
            .unwrap();
    }
}
