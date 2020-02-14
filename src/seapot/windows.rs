use super::palette;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Frame,
};

pub type Backend = CrosstermBackend<Stdout>;
pub struct Welcome {}
impl Welcome {
    pub fn draw(&self, f: &mut Frame<Backend>) {
        let text = vec![Text::raw("Welcome to Seapot!")];
        Paragraph::new(text.iter())
            .block(Block::default().title("Welcome").borders(Borders::ALL))
            .style(Style::default().fg(palette::GREEN).bg(palette::BLACK))
            .alignment(Alignment::Center)
            .render(f, f.size())
    }

    pub fn new() -> Welcome {
        Welcome {}
    }
}

pub enum Window {
    Welcome(Welcome),
}
