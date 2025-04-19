use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, widgets::TableState};
use std::time::Duration;

use crate::{
    config::BroomhildeConfig,
    ui::{Component, file_table::FileTable},
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
            let _ = terminal.draw(|frame| test.render(frame, frame.area()));

            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key.kind == KeyEventKind::Press {
                        let only_control_pressed =
                            key.modifiers.symmetric_difference(KeyModifiers::CONTROL)
                                == KeyModifiers::NONE;

                        if only_control_pressed && key.code == KeyCode::Char('c') {
                            self.active = false;
                        } else {
                            test.handle_key(key);
                        }
                    }
                }
            }
        }
    }

    pub fn execute(&mut self) {
        todo!()
    }
}

impl Default for App {
    fn default() -> Self {
        Self { active: false }
    }
}
