mod seapot;
use clap::{App, Arg};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use seapot::{musicplayer::MusicPlayer, Seapot, Window};
use std::{io, sync::mpsc, thread, time::Duration};

struct PlayerCommand {
    command: Box<dyn Fn(&mut MusicPlayer) + Send>,
}
impl PlayerCommand {
    pub fn new<F>(f: F) -> PlayerCommand
    where
        F: Fn(&mut MusicPlayer) + 'static + Send,
    {
        PlayerCommand {
            command: Box::new(f),
        }
    }
    pub fn call(self, p: &mut MusicPlayer) {
        (self.command)(p);
    }
}
fn main() {
    //Parsing cli args
    let config_cli = App::new("Seapot")
        .version("0.0.1")
        .author("Uğurcan Akkök - akkokugurcan@gmail.com")
        .about("Listen to spotify from the conformity of your favorite terminal")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .env("SPOTIFY_USERNAME")
                .help("Your spotify account's username")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .env("SPOTIFY_PASSWORD")
                .help("Your spotify account's password")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let username = match config_cli.value_of("username") {
        Some(u) => u.to_string(),
        None => panic!("No username available"),
    };

    let password = match config_cli.value_of("password") {
        Some(p) => p.to_string(),
        None => panic!("No password available"),
    };

    let mut app = Seapot::new();
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    let (event_sender, event_reciever) = mpsc::channel();
    let (player_tx, player_rx) = mpsc::channel::<PlayerCommand>();
    thread::spawn(move || {
        let mut player = MusicPlayer::new(username, password);
        loop {
            if let Ok(cmd) = player_rx.recv() {
                cmd.call(&mut player);
            }
        }
    });
    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(250)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                event_sender.send(key).unwrap();
            }
        }
    });

    loop {
        app.draw();
        if let Ok(key) = event_reciever.recv() {
            match key.code {
                KeyCode::Char('q') => {
                    terminal::disable_raw_mode().unwrap();
                    stdout.execute(LeaveAlternateScreen).unwrap();
                    return;
                }
                KeyCode::Char('s') => {
                    app.get_liked_songs_more();
                }
                KeyCode::Enter => match app.wm.focused {
                    Window::LikedSongs => {
                        let track = &app.wm.liked_songs.song_list[app.wm.liked_songs.cursor];
                        let track = track.track.uri.clone();
                        let cmd = PlayerCommand::new(move |player: &mut MusicPlayer| {
                            player.player.stop();
                            player.play_track(track.as_str());
                        });
                        player_tx.send(cmd).unwrap();
                    }
                    _ => app.homepage(),
                },
                key => app.process_key(key),
            }
        }
    }
}
