mod seapot;

use std::{
    io::{self, /*Write*/},
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::event::{
    self, 
//    poll, 
    Event, 
    KeyCode, 
//    KeyEvent
};
//use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
//use tui::backend::CrosstermBackend;
//use tui::layout::{Alignment, Constraint, Direction, Layout};
//use tui::style::{Color, Modifier, Style};
//use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
//use tui::Terminal;

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(250)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                tx.send(key).unwrap();
            }
        }
    });

    let mut app = seapot::Seapot::new();
    loop {
        app.draw();
        match rx.recv() {
            Ok(key) => match key.code {
                KeyCode::Char('q') => {
                    terminal::disable_raw_mode().unwrap();
                    stdout.execute(LeaveAlternateScreen).unwrap();
                    return;
                }
                _ => (),
            },
            _ => (),
        }
    }
}
