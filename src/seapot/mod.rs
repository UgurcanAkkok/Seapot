use futures::sync::mpsc::UnboundedReceiver;
use std::io::{self};

use tui::{backend::CrosstermBackend, Terminal};

use librespot::{
    core::{
        authentication::Credentials,
        config::SessionConfig,
        session::Session,
        spotify_id::{SpotifyId, SpotifyIdError},
    },
    playback::{
        audio_backend,
        config::PlayerConfig,
        player::{Player, PlayerEvent},
    },
};
use tokio_core::reactor::Core;

mod palette;
mod windows;
use windows::*;

pub struct Seapot {
    terminal: Terminal<Backend>,
    page: Vec<Window>,
    player: Player,
    session: Session,
    state: UnboundedReceiver<PlayerEvent>,
    core: Core,
}

impl Seapot {
    pub fn new(user: String, pass: String) -> Seapot {
        let output = io::stdout();
        let backend = CrosstermBackend::new(output);
        let terminal = Terminal::new(backend).unwrap();
        let page = vec![Window::Welcome(windows::Welcome::new())];

        let mut core = Core::new().unwrap();
        let core_handle = core.handle();
        let session_config = SessionConfig::default();
        let player_config = PlayerConfig::default();
        //let creds = Credentials::with_password(user, pass);
        let creds = Credentials::with_password("ucakdw".to_string(), "Omuzomuza123".to_string());
        let backend = audio_backend::find(None).unwrap();
        println!("Connecting");
        let session = core
            .run(Session::connect(session_config, creds, None, core_handle))
            .unwrap();
        println!("Was able to connect!");

        let (player, state) = Player::new(player_config, session.clone(), None, move || {
            (backend)(None)
        });

        Seapot {
            terminal,
            page,
            player,
            session,
            state,
            core,
        }
    }

    pub fn draw(&mut self) {
        // Unnecessary clear every time draw called?
        self.terminal.clear().unwrap();
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

    pub fn play_track(&mut self, id: &str) {
        let track = SpotifyId::from_base62(id)
            .or_else(|e| {
                eprintln!("Wrong id for track. Err: {:?}", e);
                Err(e)
            })
            .unwrap();
        self.core.run(self.player.load(track, true, 0))
            .expect("Can not load and play the song");
    }
}
