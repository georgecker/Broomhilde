use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    layout::Constraint,
    prelude::Rect,
    style::{Color, Style, Stylize},
    widgets::{Cell, HighlightSpacing, Row, Table, TableState},
};

use crate::config::FolderConfig;

use super::Component;

const COLOR_SELECTED_FG: Color = Color::Black;
const COLOR_SELECTED_BG: Color = Color::LightYellow;

pub struct FileTable {
    state: TableState,
    items: Vec<FolderConfig>,
}

impl FileTable {
    pub fn new(state: TableState, items: Vec<FolderConfig>) -> Self {
        Self { state, items }
    }
}

impl Component for FileTable {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let header = ["Path", "Stratergy"]
            .into_iter()
            .map(|h| Cell::from(h))
            .collect::<Row>()
            .height(1);
        let rows = self.items.iter().map(|item| {
            let path = Cell::from(item.path.clone());
            let stratergy = Cell::from(
                item.stratergy
                    .as_ref()
                    .map_or("".to_string(), |s| s.to_string()),
            );
            Row::new(vec![path, stratergy]).height(2)
        });

        let table = Table::new(
            rows,
            [Constraint::Percentage(75), Constraint::Percentage(25)],
        )
        .header(header)
        .cell_highlight_style(
            Style::new()
                .fg(COLOR_SELECTED_FG)
                .bg(COLOR_SELECTED_BG)
                .bold(),
        )
        .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(table, area, &mut self.state);
    }

    // TODO -> handle insert
    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('h') | KeyCode::Left => self.state.select_previous_column(),
                KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
                KeyCode::Char('l') | KeyCode::Right => self.state.select_next_column(),
                _ => {}
            }
        }
    }
}

impl Default for FileTable {
    fn default() -> Self {
        Self {
            state: TableState::default(),
            items: Vec::new(),
        }
    }
}
