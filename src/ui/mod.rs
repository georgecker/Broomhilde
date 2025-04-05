// Set target folder
// Set Stratergy

use std::io;

use crossterm::{event, terminal};
use ratatui::{DefaultTerminal, Frame};

pub fn run_ui() {
    let terminal = ratatui::init();
    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) -> Result<(), io::Error> {
    loop {
        let _ = terminal.draw(render)?;
        if let Ok(event) = event::read() {}
    }
}

fn render(frame: &mut Frame) {}
