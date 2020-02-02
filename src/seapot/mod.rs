use std::io::{self};

use tui::{
    backend::CrosstermBackend,
    //layout::{Alignment, Constraint, Direction, Layout, Rect},
    //style::{Modifier, Style},
    //widgets::{Block, Borders, Paragraph, Text, Widget},
    //Frame,
    Terminal,
};

mod palette;
mod windows;
use windows::*;

pub struct Seapot {
    terminal: Terminal<Backend>,
    page: Vec<Window>,
}

impl Seapot {
    pub fn new() -> Seapot {
        let output = io::stdout();
        let backend = CrosstermBackend::new(output);
        let terminal = Terminal::new(backend).unwrap();
        let page = vec![Window::Welcome(windows::Welcome::new())];

        Seapot { terminal, page }
    }

//    pub fn terminal(&self) -> &Terminal<Backend> {
//        &self.terminal
//    }
    pub fn draw(&mut self) {
        // Unnecessary clear?
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
}
