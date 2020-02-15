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
use clap::{Arg, App,};
use seapot::{Seapot, MusicPlayer};

fn main() {
    //Parsing cli args
    let config_cli = App::new("Seapot")
        .version("0.0.1")
        .author("Uğurcan Akkök - akkokugurcan@gmail.com")
        .about("Listen to spotify from the conformity of your favorite terminal")
        .arg(Arg::with_name("username")
             .short("u")
             .long("username")
             .env("SPOTIFY_USERNAME")
             .help("Your spotify account's username")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("password")
             .short("p")
             .long("password")
             .env("SPOTIFY_PASSWORD")
             .help("Your spotify account's password")
             .takes_value(true)
             .required(true))
        .get_matches();

    let username = match config_cli.value_of("username") {
        Some(u) => u.to_string(),
        None => panic!("No username available"),
    };

    let password = match config_cli.value_of("password") {
        Some(p) => p.to_string(),
        None => panic!("No password available"),
    };

    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
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
        if let Ok(key) = event_reciever.try_recv() {
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
