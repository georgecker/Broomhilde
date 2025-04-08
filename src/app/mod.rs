use std::{io, time::Duration};

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Terminal,
    widgets::{TableState, Widget},
};

use crate::{
    config::{self, BroomhildeConfig, FolderConfig, Stratergy},
    ui::{ConfigUi, file_table::FileTable},
};

pub struct App {
    active: bool,
}

impl App {
    pub fn run_ui(&mut self, terminal: &mut DefaultTerminal) {
        // let mut config = config::get_config().unwrap_or(BroomhildeConfig::default());
        let config = BroomhildeConfig::test_data();
        let mut test = FileTable::new(
            TableState::default().with_selected_cell(Some((0, 0))),
            config.folder_configs.clone(),
        );
        self.active = true;
        while self.active {
            terminal.draw(|frame| test.render(frame, frame.area()));

            if event::poll(Duration::from_millis(100)).unwrap() {
                self.active = !matches!(event::read().unwrap(), Event::Key(_));
            }
        }
    }

    pub fn execute(&mut self) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl Default for App {
    fn default() -> Self {
        Self { active: false }
    }
}
