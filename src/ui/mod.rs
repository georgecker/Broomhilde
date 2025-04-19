pub mod file_table;

use crossterm::event::KeyEvent;
use ratatui::{Frame, prelude::Rect};

pub trait Component {
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_key(&mut self, key: KeyEvent);
}
