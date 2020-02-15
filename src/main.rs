mod seapot;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{
    env, io,
    sync::{mpsc, },
    thread,
    time::Duration,
};

use seapot::{Seapot, MusicPlayer};

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    let username = env::var("SPOTIFY_USERNAME")
        .expect("Can not get spotify username from environment variables.");
    let password = env::var("SPOTIFY_PASSWORD")
        .expect("Can not get spotify password from environment variables.");

    let mut app = Seapot::new();
    stdout.execute(EnterAlternateScreen).unwrap();
    thread::spawn(|| {
        let mut player = MusicPlayer::new(username, password);
        player.play_track("4uLU6hMCjMI75M1A2tKUQC");
    });
    let (event_sender, event_reciever) = mpsc::channel();
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
                _ => (),
            }
        }
    }
}
