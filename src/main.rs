mod seapot;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{env, io, sync::{mpsc, Arc, Mutex, }, thread, time::Duration};
use tokio_core::reactor::Core;

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    let username = env::var("SPOTIFY_USERNAME")
        .expect("Can not get spotify username from environment variables.");
    let password = env::var("SPOTIFY_PASSWORD")
        .expect("Can not get spotify password from environment variables.");

    let mut app = seapot::Seapot::new(username, password);
    //app.play_track("4uLU6hMCjMI75M1A2tKUQC");
    println!("Played the song, moving on");
    let (event_sender, event_reciever) = mpsc::channel();
    stdout.execute(EnterAlternateScreen).unwrap();
    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(250)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                event_sender.send(key).unwrap();
            }
        }
    });

    loop {
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
        app.draw();
    }
}
